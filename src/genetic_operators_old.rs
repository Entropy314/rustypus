use crate::core::Solution;
use crate::gatypes::{SolutionDataTypes, Real};
use rand::Rng;
use rand_distr::{StandardNormal, Distribution};
use crate::math_utils::{subtract, add, multiply, magnitude, normalize, orthogonalize, is_zero, clip};

/// Trait for mutation operations
pub trait Mutation<'a> {
    fn mutate(&self, parent: &'a Solution<'a>) -> Solution<'a>;

    fn evolve(&self, parents: &'a [Solution<'a>]) -> Vec<Solution<'a>> {
        parents.iter().map(|p| self.mutate(p)).collect()
    }
}

/// Trait for crossover operations
pub trait Crossover<'a> {
    fn crossover(&self, parent1: &'a Solution<'a>, parent2: &'a Solution<'a>) -> (Solution<'a>, Solution<'a>);

    fn evolve(&self, parents: &'a [Solution<'a>]) -> Vec<Solution<'a>> {
        parents.chunks_exact(2).flat_map(|pair| {
            let (child1, child2) = self.crossover(&pair[0], &pair[1]);
            vec![child1, child2]
        }).collect()
    }
}

/// Trait for variation operations
pub trait Variation<'a> {
    fn evolve(&self, parents: &'a [Solution<'a>]) -> Vec<Solution<'a>>;
}

// Bit Flip Mutation operator
pub struct BitFlipMutation {
    pub probability: f64,
}

impl BitFlipMutation {
    pub fn new(probability: Option<f64>) -> Self {
        Self {
            probability: probability.unwrap_or(1.0),
        }
    }
}

impl Default for BitFlipMutation {
    fn default() -> Self {
        Self {
            probability: 0.1,
        }
    }
}

impl<'a> Mutation<'a> for BitFlipMutation {
    fn mutate(&self, parent: &'a Solution<'a>) -> Solution<'a> {
        let mut child = parent.clone();
        let mut rng = rand::thread_rng();

        for (i, solution_type) in parent.problem.solution_data_types.iter().enumerate() {
            if let SolutionDataTypes::BitBinary(_) = solution_type {
                if rng.gen::<f64>() < self.probability {
                    child.solution[i] = 1.0 - child.solution[i]; // Flip the bit
                }
            }
        }

        child.feasible = false;
        child.evaluated = false;
        child
    }
}

/// Polynomial Mutation operator
pub struct PolynomialMutation {
    pub probability: f64,
    pub distribution_index: f64,
}

impl Default for PolynomialMutation {
    fn default() -> Self {
        Self {
            probability: 1.0,
            distribution_index: 20.0,
        }
    }
}

impl PolynomialMutation {
    pub fn new(probability: Option<f64>, distribution_index: Option<f64>) -> Self {
        Self {
            probability: probability.unwrap_or(1.0),
            distribution_index: distribution_index.unwrap_or(20.0),
        }
    }

    fn polynomial_mutation(&self, x: f64, lower_bound: f64, upper_bound: f64) -> f64 {
        let mut rng = rand::thread_rng();
        let u = rng.gen::<f64>();
        let delta = if u < 0.5 {
            let bl = (x - lower_bound) / (upper_bound - lower_bound);
            let b = 2.0 * u + (1.0 - 2.0 * u) * (1.0 - bl).powf(self.distribution_index + 1.0);
            b.powf(1.0 / (self.distribution_index + 1.0)) - 1.0
        } else {
            let bu = (upper_bound - x) / (upper_bound - lower_bound);
            let b = 2.0 * (1.0 - u) + 2.0 * (u - 0.5) * (1.0 - bu).powf(self.distribution_index + 1.0);
            b.powf(1.0 / (self.distribution_index + 1.0)) - 1.0
        };

        x + delta * (upper_bound - lower_bound)
    }
}

impl<'a> Mutation<'a> for PolynomialMutation {
    fn mutate(&self, parent: &'a Solution<'a>) -> Solution<'a> {
        let mut child = parent.clone();
        let mut rng = rand::thread_rng();

        for (i, solution_type) in parent.problem.solution_data_types.iter().enumerate() {
            if let SolutionDataTypes::Real(real) = solution_type {
                if rng.gen::<f64>() < self.probability {
                    let lower = real.lower_bound.unwrap_or(f64::MIN);
                    let upper = real.upper_bound.unwrap_or(f64::MAX);
                    child.solution[i] = self.polynomial_mutation(child.solution[i], lower, upper);
                }
            }
        }

        child.evaluated = false;
        child.feasible = false;
        child
    }
}

impl<'a> Variation<'a> for PolynomialMutation {
    fn evolve(&self, parents: &'a [Solution<'a>]) -> Vec<Solution<'a>> {
        Mutation::evolve(self, parents)
    }
}

/// Simulated Binary Crossover (SBX) operator
pub struct SimulatedBinaryCrossover {
    pub probability: f64,
    pub distribution_index: f64,
}

impl SimulatedBinaryCrossover {
    pub fn new(probability: Option<f64>, distribution_index: Option<f64>) -> Self {
        Self {
            probability: probability.unwrap_or(1.0),
            distribution_index: distribution_index.unwrap_or(20.0),
        }
    }

    fn sbx_crossover(&self, x1: f64, x2: f64, lower: f64, upper: f64) -> (f64, f64) {
        let mut rng = rand::thread_rng();
        if (x2 - x1).abs() > f64::EPSILON {
            let y1 = x1.min(x2);
            let y2 = x1.max(x2);

            let beta = 1.0 + 2.0 * (y1 - lower) / (y2 - y1);
            let alpha = 2.0 - beta.powf(-(self.distribution_index + 1.0));
            let rand = rng.gen::<f64>();

            let betaq = if rand <= 1.0 / alpha {
                (rand * alpha).powf(1.0 / (self.distribution_index + 1.0))
            } else {
                (1.0 / (2.0 - rand * alpha)).powf(1.0 / (self.distribution_index + 1.0))
            };

            let mut c1 = 0.5 * ((y1 + y2) - betaq * (y2 - y1));
            let mut c2 = 0.5 * ((y1 + y2) + betaq * (y2 - y1));

            if rng.gen::<bool>() {
                std::mem::swap(&mut c1, &mut c2);
            }

            (clip(c1, lower, upper), clip(c2, lower, upper))
        } else {
            (x1, x2)
        }
    }
}

impl<'a> Crossover<'a> for SimulatedBinaryCrossover {
    fn crossover(&self, parent1: &'a Solution<'a>, parent2: &'a Solution<'a>) -> (Solution<'a>, Solution<'a>) {
        let mut child1 = parent1.clone();
        let mut child2 = parent2.clone();

        for (i, solution_type) in parent1.problem.solution_data_types.iter().enumerate() {
            if let SolutionDataTypes::Real(real) = solution_type {
                if rand::thread_rng().gen::<f64>() < self.probability {
                    let lower = real.lower_bound.unwrap_or(f64::MIN);
                    let upper = real.upper_bound.unwrap_or(f64::MAX);
                    let (c1, c2) = self.sbx_crossover(child1.solution[i], child2.solution[i], lower, upper);
                    child1.solution[i] = c1;
                    child2.solution[i] = c2;
                }
            }
        }

        child1.evaluated = false;
        child1.feasible = false;
        child2.evaluated = false;
        child2.feasible = false;

        (child1, child2)
    }
}

impl<'a> Variation<'a> for SimulatedBinaryCrossover {
    fn evolve(&self, parents: &'a [Solution<'a>]) -> Vec<Solution<'a>> {
        Crossover::evolve(self, parents)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Problem, Solution};
    use crate::gatypes::{SolutionDataTypes, BitBinary, Integer, Real};
    use crate::benchmark_objective_functions::parabloid_5_loc;

    #[test]
    fn test_polynomial_mutation_applies_changes() {
        let solution_data_types = vec![
            SolutionDataTypes::Real(Real::new(Some(10.0), Some(20.0))),
            SolutionDataTypes::Real(Real::new(Some(0.0), Some(50.0))),
        ];

        let problem = Problem::new(
            2,
            1,
            None,
            None,
            None,
            solution_data_types,
            parabloid_5_loc,
        );

        let solution = Solution::new(&problem);
        let mutation = PolynomialMutation::new(Some(1.0), Some(20.0));

        let mutated_solution = mutation.mutate(&solution);
        println!("Original: {:?}", solution.solution);
        println!("Mutated: {:?}", mutated_solution.solution);

        assert_ne!(solution.solution, mutated_solution.solution);
    }

    #[test]
    fn test_polynomial_mutation_respects_bounds() {
        let solution_data_types = vec![
            SolutionDataTypes::Real(Real::new(Some(0.0), Some(10.0))),
        ];

        let problem = Problem::new(
            1,
            1,
            None,
            None,
            None,
            solution_data_types,
            parabloid_5_loc,
        );

        let solution = Solution::new(&problem);
        let mutation = PolynomialMutation::new(Some(1.0), Some(20.0));

        let mutated_solution = mutation.mutate(&solution);
        let mutated_value = mutated_solution.solution[0];

        assert!(mutated_value >= 0.0 && mutated_value <= 10.0);
    }

    // #[test]
    // fn test_sbx_crossover_generates_offspring() {
    //     let solution_data_types = vec![
    //         SolutionDataTypes::Real(Real::new(Some(10.0), Some(20.0))),
    //         SolutionDataTypes::Real(Real::new(Some(15.0), Some(25.0))),
    //     ];

    //     let problem = Problem::new(
    //         2,
    //         1,
    //         None,
    //         None,
    //         None,
    //         solution_data_types,
    //         parabloid_5_loc,
    //     );

    //     let parent1 = Solution::new(&problem);
    //     let parent2 = Solution::new(&problem);
    //     let crossover = SimulatedBinaryCrossOver::new(Some(1.0), Some(20.0));

    //     let (child1, child2) = crossover.crossover(&parent1, &parent2);
    //     println!("Parent 1: {:?}", parent1.solution);
    //     println!("Parent 2: {:?}", parent2.solution);
    //     println!("Child 1: {:?}", child1.solution);
    //     println!("Child 2: {:?}", child2.solution);

    //     assert_ne!(parent1.solution, child1.solution);
    //     assert_ne!(parent2.solution, child2.solution);
    // }

    #[test]
    fn test_bitflip_mutation_applies_changes() {
        let solution_data_types = vec![
            SolutionDataTypes::BitBinary(BitBinary::new()),
            SolutionDataTypes::BitBinary(BitBinary::new()),
        ];

        let problem = Problem::new(
            2,
            1,
            None,
            None,
            None,
            solution_data_types,
            parabloid_5_loc,
        );

        let solution = Solution::new(&problem);
        let mutation = BitFlipMutation::new(Some(0.5));

        let mutated_solution = mutation.mutate(&solution);
        println!("Original: {:?}", solution.solution);
        println!("Mutated: {:?}", mutated_solution.solution);

        assert_ne!(solution.solution, mutated_solution.solution);
    }

    #[test]
    fn test_bitflip_mutation_respects_bit_values() {
        let solution_data_types = vec![
            SolutionDataTypes::BitBinary(BitBinary::new()),
        ];

        let problem = Problem::new(
            1,
            1,
            None,
            None,
            None,
            solution_data_types,
            parabloid_5_loc,
        );

        let solution = Solution::new(&problem);
        let mutation = BitFlipMutation::new(Some(1.0));

        let mutated_solution = mutation.mutate(&solution);
        let mutated_value = mutated_solution.solution[0];

        assert!(mutated_value == 0.0 || mutated_value == 1.0);
    }

    #[test]
    fn test_parent_centric_crossover_generates_child() {
        let solution_data_types = vec![
            SolutionDataTypes::Real(Real::new(Some(10.0), Some(20.0))),
            SolutionDataTypes::Real(Real::new(Some(15.0), Some(25.0))),
        ];

        let problem = Problem::new(
            2,
            1,
            None,
            None,
            None,
            solution_data_types,
            parabloid_5_loc,
        );

        let parents = vec![
            Solution::new(&problem),
            Solution::new(&problem),
            Solution::new(&problem),
        ];

        let pcx = ParentCentricCrossover::new(Some(3), Some(1), Some(0.1), Some(0.1));
        let child = pcx.parent_centric_crossover(&parents);

        println!("Parents: {:?}", parents.iter().map(|p| p.solution.clone()).collect::<Vec<_>>());
        println!("Child: {:?}", child.solution);

        assert!(child.solution.iter().all(|&x| x >= 10.0 && x <= 25.0));
    }

    #[test]
    fn test_mutation_evolve_generates_population() {
        let solution_data_types = vec![
            SolutionDataTypes::Real(Real::new(Some(10.0), Some(20.0))),
        ];

        let problem = Problem::new(
            1,
            1,
            None,
            None,
            None,
            solution_data_types,
            parabloid_5_loc,
        );

        let parents = vec![Solution::new(&problem), Solution::new(&problem)];
        let mutation = PolynomialMutation::new(Some(1.0), Some(20.0));

        let offspring = mutation.evolve(&parents);
        println!("Parents: {:?}", parents.iter().map(|p| p.solution.clone()).collect::<Vec<_>>());
        println!("Offspring: {:?}", offspring.iter().map(|p| p.solution.clone()).collect::<Vec<_>>());

        assert_eq!(offspring.len(), parents.len());
        assert_ne!(parents[0].solution, offspring[0].solution);
    }

    
}
use crate::core::Solution;
use crate::gatypes::{SolutionDataTypes, SolutionType};
use rand::Rng;
use rand_distr::StandardNormal;
use crate::gatypes::Real;
// use crate::genetic_operators::SolutionDataTypes::Real;

use crate::math_utils::{subtract, add, multiply, magnitude, normalize, orthogonalize, is_zero, clip};

pub trait Mutation<'a> {
    fn mutate(&self, parent: &'a Solution<'a>) -> Solution<'a>;

    fn evolve(&self, parents: &'a [Solution<'a>]) -> Vec<Solution<'a>> {
        if parents.len() == 1 {
            vec![self.mutate(&parents[0])]
        } else {
            parents.iter().map(|p| self.mutate(p)).collect()
        }
    }
}

pub trait Crossover<'a> {
    fn crossover(&self, parent1: &'a Solution<'a>, parent2: &'a Solution<'a>) -> (Solution<'a>, Solution<'a>);

    fn evolve(&self, parents: &'a [Solution<'a>]) -> Vec<Solution<'a>> {
        let mut offspring = Vec::new();
        for pair in parents.chunks_exact(2) {
            let (child1, child2) = self.crossover(&pair[0], &pair[1]);
            offspring.push(child1);
            offspring.push(child2);
        }
        offspring
    }
}

pub trait Variation<'a> {
    fn evolve(&self, parents: &'a [Solution<'a>]) -> Vec<Solution<'a>>;
}

pub struct PolynomialMutation {
    pub probability: Option<f64>,
    pub distribution_index: Option<f64>,
}

impl PolynomialMutation {
    pub fn new(probability: Option<f64>, distribution_index: Option<f64>) -> Self {
        // if probability is None, set it to 1.0 if distribution_index is none set it to 20.
        Self {
            probability,
            distribution_index,
        }
      
    }

    pub fn polynomial_mutation(&self, x: f64, lower_bound: f64, upper_bound: f64) -> f64 {
        let mut rng = rand::thread_rng();
        let u = rng.gen::<f64>();
        let dx = upper_bound - lower_bound;
        let distribution_index = self.distribution_index.unwrap_or(20.0);

        let delta = if u < 0.5 {
            let bl = (x - lower_bound) / dx;
            let b = 2.0 * u + (1.0 - 2.0 * u) * (1.0 - bl).powf(distribution_index + 1.0);
            b.powf(1.0 / (distribution_index + 1.0)) - 1.0
        } else {
            let bu = (upper_bound - x) / dx;
            let b = 2.0 * (1.0 - u) + 2.0 * (u - 0.5) * (1.0 - bu).powf(distribution_index + 1.0);
            b.powf(1.0 / (distribution_index + 1.0)) - 1.0
        };

        x + delta * dx
    }
}

impl<'a> Mutation<'a> for PolynomialMutation {
    fn mutate(&self, parent: &'a Solution<'a>) -> Solution<'a> {
        let mut child = parent.clone();
        let problem = &child.problem;
        let mut rng = rand::thread_rng();
        let probability = self.probability.unwrap_or(1.0);
        
        for (i, solution_type) in problem.solution_data_types.iter().enumerate() {
            match solution_type {
                SolutionDataTypes::Real(real) => {
                    if rng.gen::<f64>() < probability {
                        let lower_bound = real.lower_bound.unwrap_or(f64::MIN);
                        let upper_bound = real.upper_bound.unwrap_or(f64::MAX);
                        child.solution[i] = self.polynomial_mutation(child.solution[i], lower_bound, upper_bound);
                        
                    }
                }
                _ => {}
            }
        }
        child.feasible = false;
        child.evaluated = false;
        child
    }
}

impl<'a> Variation<'a> for PolynomialMutation {
    fn evolve(&self, parents: &'a [Solution<'a>]) -> Vec<Solution<'a>> {
        Mutation::evolve(self, parents)
    }
}
// Additional code for Solution and Problem structs, and the gatypes module

pub struct SimulatedBinaryCrossOver {
    pub probability: Option<f64>,
    pub distribution_index: Option<f64>,
}

impl SimulatedBinaryCrossOver {
    pub fn new(probability: Option<f64>, distribution_index: Option<f64>) -> Self {
        Self {
            probability,
            distribution_index,
        }
    }

    fn sbx_crossover(&self, x1: f64, x2: f64, lb: f64, ub: f64) -> (f64, f64) {
        let mut rng = rand::thread_rng();
        let mut x1 = x1;
        let mut x2 = x2;

        if (x2 - x1).abs() > f64::EPSILON {
            let y1 = x1.min(x2);
            let y2 = x1.max(x2);

            let beta = 1.0 + (2.0 * (y1 - lb) / (y2 - y1));
            let alpha = 2.0 - beta.powf(-(self.distribution_index.unwrap_or(20.) + 1.0));
            let rand = rng.gen::<f64>();

            let betaq = if rand <= 1.0 / alpha {
                (rand * alpha).powf(1.0 / (self.distribution_index.unwrap_or(20.) + 1.0))
            } else {
                (1.0 / (2.0 - rand * alpha)).powf(1.0 / (self.distribution_index.unwrap_or(20.) + 1.0))
            };

            x1 = 0.5 * ((y1 + y2) - betaq * (y2 - y1));
            x2 = 0.5 * ((y1 + y2) + betaq * (y2 - y1));

            if rng.gen::<bool>() {
                std::mem::swap(&mut x1, &mut x2);
            }

            if x1 < lb {
                x1 = lb;
            } else if x1 > ub {
                x1 = ub;
            }

            if x2 < lb {
                x2 = lb;
            } else if x2 > ub {
                x2 = ub;
            }
        }

        (x1, x2)
    }
}

impl<'a> Crossover<'a> for SimulatedBinaryCrossOver {
    fn crossover(&self, parent1: &'a Solution<'a>, parent2: &'a Solution<'a>) -> (Solution<'a>, Solution<'a>) {
        let mut child1 = parent1.clone();
        let mut child2 = parent2.clone();
        let problem = &child1.problem;
        let mut rng = rand::thread_rng();
        let probability = self.probability.unwrap_or(1.0);

        for (i, solution_type) in problem.solution_data_types.iter().enumerate() {
            match solution_type {
                SolutionDataTypes::Real(real) => {
                    if rng.gen::<f64>() < probability {
                        let lower_bound = real.lower_bound.unwrap_or(f64::MIN);
                        let upper_bound = real.upper_bound.unwrap_or(f64::MAX);
                        let (x1, x2) = self.sbx_crossover(child1.solution[i], child2.solution[i], lower_bound, upper_bound);
                        child1.solution[i] = x1;
                        child2.solution[i] = x2;
                    }
                }
                _ => {}
            }
        }

        child1.feasible = false;
        child1.evaluated = false;
        child2.feasible = false;
        child2.evaluated = false;

        (child1, child2)
    }
}

impl<'a> Variation<'a> for SimulatedBinaryCrossOver {
    fn evolve(&self, parents: &'a [Solution<'a>]) -> Vec<Solution<'a>> {
        Crossover::evolve(self, parents)
    }
}

// PCX (Parent Centric Crossover) implementation
pub struct ParentCentricCrossover {
    pub n_parents: Option<usize>,
    pub n_offspring: Option<usize>,
    pub eta: Option<f64>,
    pub zeta: Option<f64>
}
impl ParentCentricCrossover {
    pub fn new(n_parents: Option<usize>, n_offspring: Option<usize>, eta: Option<f64>, zeta: Option<f64>) -> Self {
        let n_parents: usize = n_parents.unwrap_or(10);
        let n_offspring: usize = n_offspring.unwrap_or(10);
        let eta: f64 = eta.unwrap_or(0.1);
        let zeta: f64 = zeta.unwrap_or(0.1);
        Self {
            n_parents: Some(n_parents),
            n_offspring: Some(n_offspring),
            eta: Some(eta),
            zeta: Some(zeta),
        }
    }

    fn parent_centric_crossover<'a>(&self, parents: &[Solution<'a>]) -> Solution<'a> {
        let k = parents.len();
        let n = parents[0].problem.solution_length;
        let mut rng = rand::thread_rng();

        let mut x = vec![vec![0.0; n]; k];
        for i in 0..k {
            x[i] = parents[i].solution.clone();
        }

        let g: Vec<f64> = (0..n)
            .map(|j| x.iter().map(|xi| xi[j]).sum::<f64>() / k as f64)
            .collect();

        let mut e_eta = vec![vec![0.0; n]];
        e_eta[0] = subtract(&x[k - 1], &g);

        let mut dd:f64  = 0.0;
        for i in 0..(k - 1) {
            let d = subtract(&x[i], &g);
            if !is_zero(&d) {
                let e = orthogonalize(&d, &e_eta);
                if !is_zero(&e) {
                    dd += magnitude(&e);
                    e_eta.push(normalize(&e));
                }
            }
        }
        dd /= (k - 1) as f64;

        let mut variables = x[k - 1].clone();
        variables = add(&variables, &multiply(rng.sample::<f64, StandardNormal>(StandardNormal) * self.zeta.unwrap(), &e_eta[0]));

        let eta = rng.sample::<f64, StandardNormal>(rand_distr::StandardNormal) * self.eta.unwrap();
        for i in 1..e_eta.len() {
            variables = add(&variables, &multiply(eta * dd, &e_eta[i]));
        }

        let mut result: Solution<'a> = parents[k - 1].clone();
        for j in 0..n {
            match &result.problem.solution_data_types[j] {
                SolutionDataTypes::Real(Real { lower_bound, upper_bound }) => {
                    result.solution[j] = clip(variables[j], lower_bound.unwrap_or(f64::MIN), upper_bound.unwrap_or(f64::MAX));
                }
                _ => {}
            }
        }
        result.evaluated = false;
        result.feasible = false;

        result
    }
    
}
pub struct BitFlipMutation {
    pub probability: Option<f64>,
}

impl BitFlipMutation {
    pub fn new(probability: Option<f64>) -> Self {
        Self {
            probability,
        }
    }
}

impl<'a> Mutation<'a> for BitFlipMutation {
    fn mutate(&self, parent: &'a Solution<'a>) -> Solution<'a> {
        let mut child = parent.clone();
        let problem = &child.problem;
        let mut rng = rand::thread_rng();
        let probability = self.probability.unwrap_or(1.0);

        for (i, solution_type) in problem.solution_data_types.iter().enumerate() {
            match solution_type {
                SolutionDataTypes::BitBinary(_bit_binary) => {
                    if rng.gen::<f64>() < probability {
                        child.solution[i] = 1. - child.solution[i];
                    }
                }
                _ => {}
            }
        }

        child.feasible = false;
        child.evaluated = false;
        child
    }
}

impl<'a> Variation<'a> for BitFlipMutation {
    fn evolve(&self, parents: &'a [Solution<'a>]) -> Vec<Solution<'a>> {
        Mutation::evolve(self, parents)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Problem, Solution};
    use crate::gatypes::{SolutionDataTypes, BitBinary, Integer, Real};
    use crate::benchmark_objective_functions::parabloid_5_loc;

    #[test]
    fn test_polynomial_mutation() {
        let solution_data_types = vec![
            SolutionDataTypes::BitBinary(BitBinary::new()),
            SolutionDataTypes::Integer(Integer::new(Some(10), Some(20))),
            SolutionDataTypes::Real(Real::new(Some(10.0), Some(20.0))),
            SolutionDataTypes::Integer(Integer::new(Some(10), Some(20))),
            SolutionDataTypes::Real(Real::new(Some(10.0), Some(20.0))),
        ];

        let problem = Problem::new(
            5,
            1,
            None,
            None,
            None,
            solution_data_types,
            parabloid_5_loc,
        );

        let solution = Solution::new(&problem);
        println!("Original solution: {:?}", solution.solution);
        let mutation = PolynomialMutation::new(Some(1.0), Some(20.0));
        let mutated_solution = mutation.mutate(&solution);
        println!("Mutated solution: {:?}", mutated_solution.solution);

        assert_ne!(solution.solution, mutated_solution.solution);
    }

    #[test]
    fn test_polynomial_mutation_no_change() {
        let solution_data_types = vec![
            SolutionDataTypes::BitBinary(BitBinary::new()),
            SolutionDataTypes::Integer(Integer::new(Some(10), Some(20))),
            SolutionDataTypes::Integer(Integer::new(Some(12), Some(20))),
            SolutionDataTypes::Integer(Integer::new(Some(10), Some(20))),
            SolutionDataTypes::Integer(Integer::new(Some(-10), Some(20))),
        ];

        let problem = Problem::new(
            5,
            1,
            None,
            None,
            None,
            solution_data_types,
            parabloid_5_loc,
        );

        let solution = Solution::new(&problem);
        let mutation = PolynomialMutation::new(Some(0.0), Some(20.0));
        let mutated_solution = mutation.mutate(&solution);
        println!("Mutated solution: {:?}", mutated_solution.solution);
        assert_eq!(solution.solution[0], mutated_solution.solution[0]);
    }

    // SBX tests
    #[test]
    fn test_sbx_crossover() {
        let solution_data_types = vec![
            SolutionDataTypes::Real(Real::new(Some(10.0), Some(20.0))),
            SolutionDataTypes::Real(Real::new(Some(10.0), Some(20.0))),
            SolutionDataTypes::Real(Real::new(Some(10.0), Some(20.0))),
            SolutionDataTypes::Real(Real::new(Some(10.0), Some(20.0))),
            SolutionDataTypes::Real(Real::new(Some(10.0), Some(20.0))),
        ];

        let problem = Problem::new(
            5,
            1,
            None,
            None,
            None,
            solution_data_types,
            parabloid_5_loc,
        );

        let solution1 = Solution::new(&problem);
        let solution2 = Solution::new(&problem);
        println!("Parent 1: {:?}", solution1.solution);
        println!("Parent 2: {:?}", solution2.solution);

        let sbx = SimulatedBinaryCrossOver::new(Some(1.0), Some(20.0));
        let (child1, child2) = sbx.crossover(&solution1, &solution2);
        println!("Child 1: {:?}", child1.solution);
        println!("Child 2: {:?}", child2.solution);

        assert_ne!(solution1.solution, child1.solution);
        assert_ne!(solution2.solution, child2.solution);
    }

    #[test] // Mixed solution types
    fn test_sbx_crossover_mixed() {
        let solution_data_types = vec![
            SolutionDataTypes::BitBinary(BitBinary::new()),
            SolutionDataTypes::Integer(Integer::new(Some(10), Some(20))),
            SolutionDataTypes::Real(Real::new(Some(10.0), Some(20.0))),
            SolutionDataTypes::Integer(Integer::new(Some(10), Some(20))),
            SolutionDataTypes::Real(Real::new(Some(10.0), Some(20.0))),
        ];

        let problem = Problem::new(
            5,
            1,
            None,
            None,
            None,
            solution_data_types,
            parabloid_5_loc,
        );

        let solution1 = Solution::new(&problem);
        let solution2 = Solution::new(&problem);
        println!("Parent 1: {:?}", solution1.solution);
        println!("Parent 2: {:?}", solution2.solution);

        let sbx = SimulatedBinaryCrossOver::new(Some(1.0), Some(20.0));
        let (child1, child2) = sbx.crossover(&solution1, &solution2);
        println!("Child 1: {:?}", child1.solution);
        println!("Child 2: {:?}", child2.solution);

        assert_ne!(solution1.solution, child1.solution);
        assert_ne!(solution2.solution, child2.solution);
    }

    // Bitflip tests
    #[test]
    fn test_bitflip_mutation() {
        let solution_data_types = vec![ SolutionDataTypes::BitBinary(BitBinary::new()), SolutionDataTypes::BitBinary(BitBinary::new()), 
                                                                SolutionDataTypes::BitBinary(BitBinary::new()), SolutionDataTypes::BitBinary(BitBinary::new()), 
                                                                SolutionDataTypes::Real(Real::new(Some(10.0), Some(20.0)))];

        let problem = Problem::new(
            5,
            1,
            None,
            None,
            None,
            solution_data_types,
            parabloid_5_loc,
        );

        let solution = Solution::new(&problem);
        println!("Original solution: {:?}", solution.solution);
        let mutation = BitFlipMutation::new(Some(0.5));
        let mutated_solution = mutation.mutate(&solution);
        println!("Mutated solution: {:?}", mutated_solution.solution);
        // print size of solution
        // assert_ne!(solution.solution, mutated_solution.solution);

    }

    #[test] // Parent Centric Crossover
    fn test_parent_centric_crossover() {
        let solution_data_types = vec![
            SolutionDataTypes::Real(Real::new(Some(10.0), Some(20.0))),
            SolutionDataTypes::Real(Real::new(Some(10.0), Some(20.0))),
            SolutionDataTypes::Real(Real::new(Some(10.0), Some(20.0))),
            SolutionDataTypes::Real(Real::new(Some(10.0), Some(20.0))),
            SolutionDataTypes::Real(Real::new(Some(10.0), Some(20.0))),
        ];

        let problem = Problem::new(
            5,
            1,
            None,
            None,
            None,
            solution_data_types,
            parabloid_5_loc,
        );

        let solution1 = Solution::new(&problem);
        let solution2 = Solution::new(&problem);
        let solution3 = Solution::new(&problem);
        let solution4 = Solution::new(&problem);
        let solution5 = Solution::new(&problem);
        let parents = vec![solution1, solution2, solution3, solution4, solution5];

        println!("Parents: {:?}", parents.iter().map(|p| p.solution.clone()).collect::<Vec<Vec<f64>>>());

        let pcx = ParentCentricCrossover::new(Some(5), Some(10), Some(0.1), Some(0.1));
        let child = pcx.parent_centric_crossover(&parents);
        println!("Child: {:?}", child.solution);

        assert!(child.solution.iter().all(|&x| x >= 10.0 && x <= 20.0));
        // more tests
        // assert_ne!(&solution1.solution, &child.solution);


    }
    //

}
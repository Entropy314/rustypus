use crate::core::{Solution, Problem};
use crate::gatypes::{SolutionDataTypes, Real};
use rand::rngs::ThreadRng;
use std::collections::HashMap;
use std::sync::Arc;
use rand::Rng;

/// Trait for mutation operations
/// Trait for mutation operations
pub trait Mutation<'a>: Send + Sync {
    fn mutate(&self, parent: &'a Solution<'a>, index: usize) -> f64;
}
/// MutationManager to manage and apply mutations
pub struct MutationManager<'a> {
    default_mutations: HashMap<&'static str, Arc<dyn Mutation<'a>>>,
    custom_mutations: HashMap<usize, Arc<dyn Mutation<'a>>>,

}

impl<'a> MutationManager<'a> {
    /// Creates a new MutationManager with default mutations
    pub fn new() -> Self {
        let mut default_mutations: HashMap<&'static str, Arc<dyn Mutation<'a>>> = HashMap::new();
        default_mutations.insert("BitBinary", Arc::new(BitFlipMutation::default()));
        default_mutations.insert("Real", Arc::new(UniformMutation::default()));
        default_mutations.insert("Integer", Arc::new(UniformMutation::default()));

        Self {
            default_mutations,
            custom_mutations: HashMap::new(),
        }
    }

    /// Sets a custom mutation for a specific index
    pub fn set_custom_mutation(&mut self, index: usize, mutation: Arc<dyn Mutation<'a>>) {
        self.custom_mutations.insert(index, mutation);
    }

    /// Applies mutations to the parent solution and returns the mutated child
    // pub fn mutate(&self, parent: &'a Solution<'a>) -> Solution<'a> {
    //     let mut child = parent.clone();

    //     for (i, solution_type) in parent.problem.solution_data_types.iter().enumerate() {
    //         let mutation = self.custom_mutations.get(&i).cloned().or_else(|| {
    //             match solution_type {
    //                 SolutionDataTypes::BitBinary(_) => self.default_mutations.get("BitBinary").cloned(),
    //                 SolutionDataTypes::Real(_) => self.default_mutations.get("Real").cloned(),
    //                 SolutionDataTypes::Integer(_) => self.default_mutations.get("Integer").cloned(),
    //                 _ => None,
    //             }
    //         });

    //         if let Some(mutation) = mutation {
    //             child.solution[i] = mutation.mutate(parent, i);
    //         }
    //     }

    //     child.feasible = false;
    //     child.evaluated = false;
    //     child
    // }
    pub fn mutate(&self, parent: &'a Solution<'a>) -> Solution<'a> {
        let mut child = parent.clone();
        for (i, solution_type) in parent.problem.solution_data_types.iter().enumerate() {
            let mutation = self
                .custom_mutations
                .get(&i)
                .cloned()
                .or_else(|| match solution_type {
                    SolutionDataTypes::BitBinary(_) => self.default_mutations.get("BitBinary").cloned(),
                    SolutionDataTypes::Real(_) => self.default_mutations.get("Real").cloned(),
                    SolutionDataTypes::Integer(_) => self.default_mutations.get("Integer").cloned(),
                    _ => None,
                });

            if let Some(mutation) = mutation {
                child.solution[i] = mutation.mutate(parent, i);
            }
        }
        child.feasible = false;
        child.evaluated = false;
        child
    }
}


/// Helper trait to retrieve bounds for solution data types
pub trait SolutionTypeBounds {
    fn get_lower_bound(&self) -> Option<f64>;
    fn get_upper_bound(&self) -> Option<f64>;
}

impl SolutionTypeBounds for SolutionDataTypes {
    fn get_lower_bound(&self) -> Option<f64> {
        match self {
            SolutionDataTypes::Real(real) => real.lower_bound,
            SolutionDataTypes::Integer(integer) => integer.lower_bound.map(|v| v as f64),
            _ => None,
        }
    }

    fn get_upper_bound(&self) -> Option<f64> {
        match self {
            SolutionDataTypes::Real(real) => real.upper_bound,
            SolutionDataTypes::Integer(integer) => integer.upper_bound.map(|v| v as f64),
            _ => None,
        }
    }
}

/// Bit Flip Mutation
pub struct BitFlipMutation {
    pub probability: f64,
}

impl Default for BitFlipMutation {
    fn default() -> Self {
        Self { probability: 0.5 }
    }
}

impl<'a> Mutation<'a> for BitFlipMutation {
    fn mutate(&self, parent: &'a Solution<'a>, index: usize) -> f64 {
        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() < self.probability {
            1.0 - parent.solution[index]
        } else {
            parent.solution[index]
        }
    }
}

/// Uniform Mutation
pub struct UniformMutation {
    pub probability: f64,
}

impl Default for UniformMutation {
    fn default() -> Self {
        Self { probability: 1.0 }
    }
}

impl<'a> Mutation<'a> for UniformMutation {
    fn mutate(&self, parent: &'a Solution<'a>, index: usize) -> f64 {
        let mut rng = rand::thread_rng();
        match &parent.problem.solution_data_types[index] {
            SolutionDataTypes::Integer(integer) => {
                let lower_bound = integer.lower_bound.unwrap_or(i64::MIN) as f64;
                let upper_bound = integer.upper_bound.unwrap_or(i64::MAX) as f64;

                if rng.gen::<f64>() < self.probability {
                    rng.gen_range(lower_bound..=upper_bound).round() // Ensures result is an integer
                } else {
                    parent.solution[index]
                }
            }
            SolutionDataTypes::Real(real) => {
                let lower_bound = real.lower_bound.unwrap_or(f64::MIN);
                let upper_bound = real.upper_bound.unwrap_or(f64::MAX);

                if rng.gen::<f64>() < self.probability {
                    rng.gen_range(lower_bound..=upper_bound)
                } else {
                    parent.solution[index]
                }
            }
            _ => parent.solution[index], // No mutation for other types
        }
    }
}

pub struct PolynomialMutation {
    pub probability: f64,
    pub distribution_index: f64,
}

impl PolynomialMutation {
    pub fn new(probability: Option<f64>, distribution_index: Option<f64>) -> Self {
        Self {
            probability: probability.unwrap_or(1.0),
            distribution_index: distribution_index.unwrap_or(20.0),
        }
    }
}

impl<'a> Mutation<'a> for PolynomialMutation {
    fn mutate(&self, parent: &'a Solution<'a>, index: usize) -> f64 {
        let mut rng = rand::thread_rng();
        match &parent.problem.solution_data_types[index] {
            SolutionDataTypes::Integer(integer) => {
                let lower_bound = integer.lower_bound.unwrap_or(i64::MIN) as f64;
                let upper_bound = integer.upper_bound.unwrap_or(i64::MAX) as f64;

                if rng.gen::<f64>() < self.probability {
                    let u = rng.gen::<f64>();
                    let dx = upper_bound - lower_bound;
                    let delta = if u < 0.5 {
                        let bl = (parent.solution[index] - lower_bound) / dx;
                        (2.0 * u + (1.0 - 2.0 * u) * (1.0 - bl).powf(self.distribution_index + 1.0))
                            .powf(1.0 / (self.distribution_index + 1.0))
                            - 1.0
                    } else {
                        let bu = (upper_bound - parent.solution[index]) / dx;
                        (2.0 * (1.0 - u) + 2.0 * (u - 0.5) * (1.0 - bu).powf(self.distribution_index + 1.0))
                            .powf(1.0 / (self.distribution_index + 1.0))
                            - 1.0
                    };
                    (parent.solution[index] + delta * dx).round().clamp(lower_bound, upper_bound)
                } else {
                    parent.solution[index]
                }
            }
            SolutionDataTypes::Real(real) => {
                let lower_bound = real.lower_bound.unwrap_or(f64::MIN);
                let upper_bound = real.upper_bound.unwrap_or(f64::MAX);

                if rng.gen::<f64>() < self.probability {
                    let u = rng.gen::<f64>();
                    let dx = upper_bound - lower_bound;
                    let delta = if u < 0.5 {
                        let bl = (parent.solution[index] - lower_bound) / dx;
                        (2.0 * u + (1.0 - 2.0 * u) * (1.0 - bl).powf(self.distribution_index + 1.0))
                            .powf(1.0 / (self.distribution_index + 1.0))
                            - 1.0
                    } else {
                        let bu = (upper_bound - parent.solution[index]) / dx;
                        (2.0 * (1.0 - u) + 2.0 * (u - 0.5) * (1.0 - bu).powf(self.distribution_index + 1.0))
                            .powf(1.0 / (self.distribution_index + 1.0))
                            - 1.0
                    };
                    (parent.solution[index] + delta * dx).clamp(lower_bound, upper_bound)
                } else {
                    parent.solution[index]
                }
            }
            _ => parent.solution[index], // No mutation for other types
        }
    }
}

/// Gaussian Mutation
pub struct GaussianMutation {
    pub probability: f64,
    pub standard_deviation: f64,
}

impl GaussianMutation {
    pub fn new(probability: Option<f64>, standard_deviation: Option<f64>) -> Self {
        Self {
            probability: probability.unwrap_or(1.0),
            standard_deviation: standard_deviation.unwrap_or(0.1),
        }
    }
}

impl<'a> Mutation<'a> for GaussianMutation {
    fn mutate(&self, parent: &'a Solution<'a>, index: usize) -> f64 {
        let mut rng = rand::thread_rng();
        let lower_bound = parent.problem.solution_data_types[index]
            .get_lower_bound()
            .unwrap_or(f64::MIN);
        let upper_bound = parent.problem.solution_data_types[index]
            .get_upper_bound()
            .unwrap_or(f64::MAX);

        if rng.gen::<f64>() < self.probability {
            (parent.solution[index] + rng.gen::<f64>() * self.standard_deviation)
                .clamp(lower_bound, upper_bound)
        } else {
            parent.solution[index]
        }
    }
}
// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Problem, Solution};
    use crate::gatypes::{SolutionDataTypes, Real, Integer, BitBinary};

    fn setup_problem() -> Problem {
        Problem {
            solution_length: 5,
            number_of_objectives: 1,
            objective_constraint: Some(vec![Some(10.0)]),
            objective_constraint_operands: Some(vec![Some("<".to_string())]),
            direction: Some(vec![1]),
            solution_data_types: vec![
                SolutionDataTypes::BitBinary(BitBinary::new()),
                SolutionDataTypes::Integer(Integer::new(Some(-2000), Some(2000))),
                SolutionDataTypes::Real(Real::new(Some(-100.0), Some(1000.0))),
                SolutionDataTypes::Real(Real::new(Some(-100.0), Some(1000.0))),
                SolutionDataTypes::Real(Real::new(Some(-100.0), Some(1000.0))),
            ],
            objective_function: |x| vec![x.iter().sum()],
        }
    }

    fn setup_solution(problem: &Problem) -> Solution {
        
        Solution {
            problem,
            solution: vec![1.0, 10.0, 10.0, 10.0, 10.0],
            objective_fitness_values: Vec::with_capacity(*problem.number_of_objectives()),
            constraint_values: Vec::with_capacity(*problem.number_of_objectives()),
            constraint_violation: 0,
            feasible: false,
            evaluated: false,
        }
    }
    

     #[test]
    fn test_default_mutation_manager() {
        let problem = setup_problem(); 
        
        let parent = setup_solution(&problem).clone();
        {
            let mutation_manager = MutationManager::new();
            let child = mutation_manager.mutate(&parent);
            println!(" Parent: {:?}", parent.solution);
            println!(" Child: {:?}", child.solution);
            assert!(child.solution[0] == 0.0 || child.solution[0] == 1.0);
            assert!(child.solution[1] != parent.solution[1]);
            assert!(child.solution[2] != parent.solution[2]);
            assert!(child.solution[3] != parent.solution[3]);
            assert!(child.solution[4] != parent.solution[4]);
        }
    }

    #[test]
    fn test_bit_flip_mutation() {
        let problem = setup_problem(); 
        let mutation = BitFlipMutation{probability: 1.0};
        let parent1 = setup_solution(&problem);
        let mut parent2 = setup_solution(&problem);
        parent2.solution[0] = 0.0;
        let child1 = mutation.mutate(&parent1, 0);
        let child2 = mutation.mutate(&parent2, 0);
        assert!(child1 == 0.0);
        assert!(child2 == 1.0);
    }
    #[test]
    fn test_polynomial_mutation_with_integer_and_real() {
        let problem = setup_problem(); 

        let parent = setup_solution(&problem);


        let mutation = PolynomialMutation::new(Some(1.0), Some(20.0));
        let child_solution_0 = mutation.mutate(&parent, 1); // Integer mutation
        let child_solution_1 = mutation.mutate(&parent, 2); // Real mutation
        let child_solution_2 = mutation.mutate(&parent, 3); // Real mutation
        let child_solution_3 = mutation.mutate(&parent, 4); // Real mutation

        assert!(child_solution_0 >= -2000.0 && child_solution_0 <= 2000.0);
        assert!(child_solution_1 >= -100.0 && child_solution_1 <= 1000.0);
        assert!(child_solution_2 >= -100.0 && child_solution_2 <= 1000.0);
        assert!(child_solution_3 >= -100.0 && child_solution_3 <= 1000.0);
        // assert new solution is mutated
        assert!(child_solution_0 != parent.solution[1]);
        assert!(child_solution_1 != parent.solution[2]);
        assert!(child_solution_2 != parent.solution[3]);
        assert!(child_solution_3 != parent.solution[4]);
        
    }

    #[test]
    fn test_uniform_mutation_with_integer_and_real() {
        let problem = setup_problem(); 

        let parent = setup_solution(&problem);

        let mutation = UniformMutation::default();
        let child_solution_0 = mutation.mutate(&parent, 1); // Integer mutation
        let child_solution_1 = mutation.mutate(&parent, 2); // Real mutation
        let child_solution_2 = mutation.mutate(&parent, 3); // Real mutation
        let child_solution_3 = mutation.mutate(&parent, 4); // Real mutation

        assert!(child_solution_0 >= -2000.0 && child_solution_0 <= 2000.0);
        assert!(child_solution_1 >= -100.0 && child_solution_1 <= 1000.0);
        assert!(child_solution_2 >= -100.0 && child_solution_2 <= 1000.0);
        assert!(child_solution_3 >= -100.0 && child_solution_3 <= 1000.0);
        // assert new solution is mutated
        assert!(child_solution_0 != parent.solution[1]);
        assert!(child_solution_1 != parent.solution[2]);
        assert!(child_solution_2 != parent.solution[3]);
        assert!(child_solution_3 != parent.solution[4]);
    
    }

    #[test]
    fn test_gaussian_mutation_with_integer_and_real() {
        let problem = setup_problem(); 

        let parent = setup_solution(&problem);

        let mutation = GaussianMutation::new(Some(1.0), Some(0.1));
        let child_solution_0 = mutation.mutate(&parent, 1); // Integer mutation
        let child_solution_1 = mutation.mutate(&parent, 2); // Real mutation
        let child_solution_2 = mutation.mutate(&parent, 3); // Real mutation
        let child_solution_3 = mutation.mutate(&parent, 4); // Real mutation

        assert!(child_solution_0 >= -2000.0 && child_solution_0 <= 2000.0);
        assert!(child_solution_1 >= -100.0 && child_solution_1 <= 1000.0);
        assert!(child_solution_2 >= -100.0 && child_solution_2 <= 1000.0);
        assert!(child_solution_3 >= -100.0 && child_solution_3 <= 1000.0);
        // assert new solution is mutated
        assert!(child_solution_0 != parent.solution[1]);
        assert!(child_solution_1 != parent.solution[2]);
        assert!(child_solution_2 != parent.solution[3]);
        assert!(child_solution_3 != parent.solution[4]);
    }


}

use crate::core::{Problem, Solution};



#[derive(Debug)]
pub enum DominanceEnum  {
    ParetoDominance,
    EpsilonDominance, 
    AttributeDominance
}

pub trait Dominance { 
    fn compare_solutions(&self, solution_1: &Solution, solution_2: &Solution) -> i32;
}
#[derive(Debug)]
pub struct ParetoDominance ;

// impl<'a> ParetoDominance<'a> { 
//     pub fn new(solution_1: &'a Solution, solution_2: &'a Solution) -> Self {
//         ParetoDominance { solution_1, solution_2 }
//         }     
// }

impl Dominance for ParetoDominance {

    fn compare_solutions(&self, solution_1: &Solution, solution_2: &Solution) -> i32 {
            
        let problem: &Problem = &solution_1.problem;
        let range_iter: std::ops::Range<usize> = (0..*problem.number_of_objectives()).into_iter(); 
     
        if let Some(ref constraints) = &problem.objective_constraint { 
            if constraints.len() > 0  && (&solution_1.constraint_violation != &solution_2.constraint_violation) {
                match (&solution_1.constraint_violation, &solution_2.constraint_violation) { 
                    (0, _) => -1, 
                    (_, 0) => 1, 
                    (v1, v2) if v1 < v2 => -1, 
                    (v1, v2) if v1 > v2 => 1, 
                    _ => 0
                }; 
            }
        }
        let mut is_solution_1_better = false;
        let mut is_solution_2_better = false;
        println!("SOLUTION 1 OBJECTIVE VALUES FFFFF: {:?}",solution_1.objective_fitness_values);  
        println!("SOLUTION 2 OBJECTIVE VALUES FFFFF: {:?}",solution_2.objective_fitness_values);
        for _i in range_iter { 
            println!("INDEX {:?}", _i); 
            let mut obj_1: f64 = solution_1.objective_fitness_values[_i]; 
            let mut obj_2 = solution_2.objective_fitness_values[_i]; 
            println!("OBJ 1 {:?}", obj_1);
            println!("OBJ 2 {:?}", obj_2);

            if let Some(direction) = &problem.direction() {
                println!("DIRECTION {:?}", direction[_i]); 
                if direction[_i] == -1 {
                    obj_1 = -obj_1;
                    obj_2 = -obj_2;
                }

                if obj_1 < obj_2 {
                    println!("SOLUTION 1 IS BETTER");
                    is_solution_1_better = true;
                    if is_solution_2_better {
                        return 0;
                    }
                } else if obj_1 > obj_2 {
                    println!("SOLUTION 2 IS BETTER");
                    is_solution_2_better = true;
                    if is_solution_1_better {
                        return 0;
                    }     
                }
            }
        }

        if is_solution_1_better == is_solution_2_better {
            return 0;
        } else if is_solution_1_better {
            return -1;
        } else {
            return -1;
        }
    
    
    }
}

// pub struct AttributeDominance<'a> { 
//     larger_preferred: bool, 
//     solution_1: &'a Solution<'a>,
//     solution_2: &'a Solution<'a>

// }

// impl<'a> AttributeDominance<'a> { 
//     pub fn new(larger_preferred: bool, solution_1: &'a Solution, solution_2: &'a Solution) -> Self {
//         AttributeDominance { larger_preferred, solution_1, solution_2 }
//     }
// }

// impl<'a> Dominance for AttributeDominance<'a> {

//     fn compare_solutions(&self, solution_1: &Solution, solution_2: &Solution) -> i32 {
        
//     }


// }
// enum EpsilonEnum { 
//     Single(f64),
//     Multi(Vec<f64>)
// }

// pub struct EpsilonDominance<'a> {
//     pub solution_1: &'a Solution<'a>, 
//     pub solution_2: &'a Solution<'a>,
//     pub epsilon: Vec<f64>
// }

// impl<'a> EpsilonDominance<'a> { 
//     pub fn new(solution_1: &'a Solution, solution_2: &'a Solution, epsilon:  Vec<f64>) -> Self {
//         EpsilonDominance { solution_1, solution_2, epsilon }
//         }

//     pub fn same_box(&self, solution_1: &Solution, solution_2: &Solution) -> i8 {
//         let problem: &Problem = &solution_1.problem;
//         let range_iter: std::ops::Range<usize> = (0..*problem.number_of_objectives()).into_iter(); 

//         if let Some(ref constraints) = &problem.objective_constraint { 
//             if constraints.len() > 0  && (&solution_1.constraint_violation != &solution_2.constraint_violation) {
//                 if solution_1.constraint_violation == 0 {
//                     return 0;
//                 } else if solution_2.constraint_violation == 0 {
//                     return 0;
//                 } else if solution_1.constraint_violation < solution_2.constraint_violation {
//                     return 0;
//                 } else if solution_1.constraint_violation > solution_2.constraint_violation {
//                     return 0;
//                 }
//             }
//         }
        
//         let mut is_solution_1_better = false;
//         let mut is_solution_2_better = false;
//         for _i in range_iter { 
//             let obj_1 = &solution_1.objective_fitness_values[_i];
//             let obj_2 = &solution_2.objective_fitness_values[_i];

//             let mut obj_1_value: f64 = *obj_1;
//             let mut obj_2_value: f64 = *obj_2;
//             println!("EPSILON INDEX {:?}", _i); 
//             let obj_1: &f64 = &solution_1.objective_fitness_values[_i]; 
//             let obj_2 = &solution_2.objective_fitness_values[_i]; 
//             println!("EPSILON OBJ 1 {:?}", obj_1);
//             println!("EPSILON OBJ 2 {:?}", obj_2);

//             if let Some(direction) = &problem.direction() {
//                 println!("DIRECTION {:?}", direction[_i]); 
//                 if direction[_i] == -1 {
//                     obj_1_value = -obj_1_value;
//                     obj_2_value = -obj_2_value;
//                 }
//             }
//             let mut epsilon_prime: f64 = self.epsilon[_i];
//             // Create variable i1 and i2 that is math.floor(obj_1/epsilon) and math.floor(obj_2/epsilon)
//             let i1: f64 = (obj_1_value / &epsilon_prime).floor();
//             let i2: f64 = (obj_2 / &epsilon_prime).floor();

//             if i1 < i2 { 
//                 is_solution_1_better = true;
//                 if is_solution_2_better {
//                     return 0;
//                 }
//             } else if i1 > i2 {
//                 is_solution_2_better = true;
//                 if is_solution_1_better {
//                     return 0 ;
//                 }
            
//             }


//         }

//         // if not is_solution_1_better and is_solution_2_better return true else false
//         if !is_solution_1_better && !is_solution_2_better {
//             return 1;
//         } else {
//             return 0;
//         }


//     }

// }

// impl <'a> Dominance for EpsilonDominance<'a> {
//     fn compare_solutions(&self, solution_1: &Solution, solution_2: &Solution) -> i {
        
//         let problem: &Problem = &solution_1.problem;
//         let range_iter: std::ops::Range<usize> = (0..*problem.number_of_objectives()).into_iter(); 

//         if let Some(ref constraints) = &problem.objective_constraint { 
//             if constraints.len() > 0  && (&solution_1.constraint_violation != &solution_2.constraint_violation) {
//                 if solution_1.constraint_violation == 0 {
//                     return 0;
//                 } else if solution_2.constraint_violation == 0 {
//                     return 0;
//                 } else if solution_1.constraint_violation < solution_2.constraint_violation {
//                     return 0;
//                 } else if solution_1.constraint_violation > solution_2.constraint_violation {
//                     return 0;
//                 }
//             }
//         }
        
//         let mut is_solution_1_better = false;
//         let mut is_solution_2_better = false;
//         for _i in range_iter { 
//             let obj_1 = &solution_1.objective_fitness_values[_i];
//             let obj_2 = &solution_2.objective_fitness_values[_i];

//             let mut obj_1_value: f64 = *obj_1;
//             let mut obj_2_value: f64 = *obj_2;
//             println!("EPSILON INDEX {:?}", _i); 
//             let obj_1: &f64 = &solution_1.objective_fitness_values[_i]; 
//             let obj_2 = &solution_2.objective_fitness_values[_i]; 
//             println!("EPSILON OBJ 1 {:?}", obj_1);
//             println!("EPSILON OBJ 2 {:?}", obj_2);

//             if let Some(direction) = &problem.direction() {
//                 println!("DIRECTION {:?}", direction[_i]); 
//                 if direction[_i] == -1 {
//                     obj_1_value = -obj_1_value;
//                     obj_2_value = -obj_2_value;
//                 }
//             }
//             let mut epsilon_prime: f64 = self.epsilon[_i];
//             // Create variable i1 and i2 that is math.floor(obj_1/epsilon) and math.floor(obj_2/epsilon)
//             let i1: f64 = (obj_1_value / &epsilon_prime).floor();
//             let i2: f64 = (obj_2 / &epsilon_prime).floor();

//             if i1 < i2 { 
//                 is_solution_1_better = true;
//                 if is_solution_2_better {
//                     return 0;
//                 }
//             } else if i1 > i2 {
//                 is_solution_2_better = true;
//                 if is_solution_1_better {
//                     return 0 ;
//                 }       
//             }
//         }

//         // if not is_solution_1_better and is_solution_2_better return true else false
//         if !is_solution_1_better && !is_solution_2_better {
//             let mut dist_1: f64 = 0.0;
//             let mut dist_2: f64 = 0.0;

//             for _i in range_iter { 
//                 let obj_1 = &solution_1.objective_fitness_values[_i];
//                 let obj_2 = &solution_2.objective_fitness_values[_i];

//                 let mut obj_1_value: f64 = *obj_1;  
//                 let mut obj_2_value: f64 = *obj_2;
//                 println!("EPSILON INDEX {:?}", _i); 
//                 let obj_1: &f64 = &solution_1.objective_fitness_values[_i]; 
//                 let obj_2 = &solution_2.objective_fitness_values[_i]; 
//                 println!("EPSILON OBJ 1 {:?}", obj_1);
//                 println!("EPSILON OBJ 2 {:?}", obj_2);

//                 if let Some(direction) = &problem.direction() {
//                     println!("DIRECTION {:?}", direction[_i]); 
//                     if direction[_i] == -1 {
//                         obj_1_value = -obj_1_value;
//                         obj_2_value = -obj_2_value;
//                     }
//                 }
//                 let mut epsilon_prime: f64 = self.epsilon[_i];
//                 // Create variable i1 and i2 that is math.floor(obj_1/epsilon) and math.floor(obj_2/epsilon)
//                 let i1: f64 = (obj_1_value / &epsilon_prime).floor();
//                 let i2: f64 = (obj_2 / &epsilon_prime).floor();
//                 dist_1 += (obj_1_value - (i1 * epsilon_prime)).abs();
//                 dist_2 += (obj_2_value - (i2 * epsilon_prime)).abs();
//             }
//         } else {
//             return 0;
//         }

//         // TODO: CONTINUE EPSILON DOMINANCE!!!!


//     }
// }
    


// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Problem, Solution};
    use crate::gatypes::{SolutionDataTypes, Real, BitBinary, Integer};
    use crate::benchmark_objective_functions::{parabloid_5, parabloid_hyper_5};
    // Create Problem
   
    #[test]
    fn test_pareto_dominance_parablioid_5_dir_pos() {
        // positive direction for single objecteive function
        // R^5 => R^1
        let problem: Problem = Problem {
            solution_length: 5,
            number_of_objectives: 1,
            objective_constraint: Some(vec![Some(10.0)]),
            objective_constraint_operands: Some(vec![Some("<".to_string())]),
            direction: Some(vec![1]),
            solution_data_types: vec![SolutionDataTypes::BitBinary(BitBinary::new()), 
                                    SolutionDataTypes::Integer(Integer::new(Some(0), Some(100))),
                                    SolutionDataTypes::Real(Real::new(Some(0.), Some(100.))),
                                    SolutionDataTypes::Integer(Integer::new(Some(0), Some(100))),
                                    SolutionDataTypes::Real(Real::new(Some(0.), Some(100.))),],
            objective_function: parabloid_5
        };
        let solution_vector1:Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let solution_vector2:Vec<f64> = vec![12.0, 10.0, -3.0, 4.0, 5.0];

        let mut solution_1 = Solution {problem: &problem, solution: solution_vector1, 
                                                objective_fitness_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_violation: 0, feasible: false, evaluated: false}; 
        let mut solution_2 = Solution {problem: &problem, solution: solution_vector2, 
                                                objective_fitness_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_violation: 0, feasible: false, evaluated: false}; 

        // Evaluate Solutions
        solution_1.evaluate();
        solution_2.evaluate();
        
        // print pareto dominance values
        let result = ParetoDominance.compare_solutions(&solution_1, &solution_2); 
        println!("LLLL PARETO  {:?}  dddd", result);

        // assert_eq!(pareto_dominance.compare_solutions(&solution_1, &solution_2), -1);
    }

    #[test]
    fn test_pareto_dominance_parablioid_5_dir_neg() {
        // negative direction for single objecteive function
        // R^5 => R^1
        let problem: Problem = Problem {
            solution_length: 3,
            number_of_objectives: 1,
            objective_constraint: Some(vec![Some(10.0)]),
            objective_constraint_operands: Some(vec![Some("<".to_string())]),
            direction: Some(vec![-1]),
            solution_data_types: vec![SolutionDataTypes::BitBinary(BitBinary::new()), 
                                        SolutionDataTypes::Integer(Integer::new(Some(0), Some(100))),
                                        SolutionDataTypes::Real(Real::new(Some(0.), Some(100.))),
                                        SolutionDataTypes::Integer(Integer::new(Some(0), Some(100))),
                                        SolutionDataTypes::Real(Real::new(Some(0.), Some(100.))),],
            objective_function: parabloid_5
        };
        let solution_vector1:Vec<f64> = vec![1.0, 2.0, -3.0, 4.0, 5.0];
        let solution_vector2:Vec<f64> = vec![12.0, 10.0, -3.0, 4.0, 5.0];

        let mut solution_1 = Solution {problem: &problem, solution: solution_vector1, 
                                                objective_fitness_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_violation: 0, feasible: false, evaluated: false}; 
        let mut solution_2 = Solution {problem: &problem, solution: solution_vector2, 
                                                objective_fitness_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_violation: 0, feasible: false, evaluated: false}; 

        // Evaluate Solutions
        solution_1.evaluate();
        solution_2.evaluate();
        // print pareto dominance values
        let result = ParetoDominance.compare_solutions(&solution_1, &solution_2); 
        println!("LLLL PARETO  {:?}  dddd", result);

        // assert_eq!(pareto_dominance.compare_solutions(&solution_1, &solution_2), -1);
    }

    #[test]
    fn test_pareto_dominance_parablioid_5_multi_dir_pos() {
        // negative direction for single objecteive function
        // R^5 => R^5
        let problem: Problem = Problem {
            solution_length: 5,
            number_of_objectives: 5,
            objective_constraint: Some(vec![Some(50.0), Some(60.0), Some(70.0), Some(80.0), Some(90.0)]),
            objective_constraint_operands: Some(vec![Some("<".to_string()), Some("<".to_string()), Some("<".to_string()), Some("<".to_string()), Some("<".to_string())], ),
            direction: Some(vec![1, 1, 1, 1, 1]),
            solution_data_types: vec![SolutionDataTypes::BitBinary(BitBinary::new()), 
                                    SolutionDataTypes::Integer(Integer::new(Some(0), Some(100))),
                                    SolutionDataTypes::Real(Real::new(Some(0.), Some(100.))),
                                    SolutionDataTypes::Integer(Integer::new(Some(0), Some(100))),
                                    SolutionDataTypes::Real(Real::new(Some(0.), Some(100.))),],
            objective_function: parabloid_hyper_5
        };
        let solution_vector1:Vec<f64> = vec![1.0, 2.0, -3.0, 4.0, 5.0];
        let solution_vector2:Vec<f64> = vec![12.0, 10.0, -3.0, 4.0, 5.0];

        let mut solution_1 = Solution {problem: &problem, solution: solution_vector1, 
                                                objective_fitness_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_violation: 0, feasible: false, evaluated: false}; 
        let mut solution_2 = Solution {problem: &problem, solution: solution_vector2, 
                                                objective_fitness_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_violation: 0, feasible: false, evaluated: false}; 

        // Evaluate Solutions
        solution_1.evaluate();
        solution_2.evaluate();
        println!("SOLUTION 1 OBJECTIVE VALUES: {:?}",solution_1.objective_fitness_values);
        // print pareto dominance values
        let result = ParetoDominance.compare_solutions(&solution_1, &solution_2); 
        println!("LLLL PARETO  {:?}  dddd", result);

        // assert_eq!(pareto_dominance.compare_solutions(&solution_1, &solution_2), -1);
    }

    
    // TODO: Add Partial constraints Eg Constraint [ >1, < 5, None, None, None]
    #[test]
    fn test_pareto_dominance_parablioid_5_multi_dir_pos_par() {
        // negative direction for single objecteive function
        // R^5 => R^5
        let problem: Problem = Problem {
            solution_length: 5,
            number_of_objectives: 1,
            objective_constraint: Some(vec![Some(50.0), None, Some(70.0), Some(80.0), Some(90.0)]),
            objective_constraint_operands: Some(vec![Some("<".to_string()), None, Some("<".to_string()), Some("<".to_string()), Some("<".to_string())], ),
            direction: Some(vec![1, 1, 1, 1, 1]),
            solution_data_types: vec![SolutionDataTypes::BitBinary(BitBinary::new()), 
                                    SolutionDataTypes::Integer(Integer::new(Some(0), Some(100))),
                                    SolutionDataTypes::Real(Real::new(Some(0.), Some(100.))),
                                    SolutionDataTypes::Integer(Integer::new(Some(0), Some(100))),
                                    SolutionDataTypes::Real(Real::new(Some(0.), Some(100.))),],
            objective_function: parabloid_hyper_5
        };
        let solution_vector1:Vec<f64> = vec![1.0, 2.0, -3.0, 4.0, 5.0];
        let solution_vector2:Vec<f64> = vec![12.0, 10.0, -3.0, 4.0, 5.0];

        let mut solution_1 = Solution {problem: &problem, solution: solution_vector1, 
                                                objective_fitness_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_violation: 0, feasible: false, evaluated: false}; 
        let mut solution_2 = Solution {problem: &problem, solution: solution_vector2, 
                                                objective_fitness_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_violation: 0, feasible: false, evaluated: false}; 

        // Evaluate Solutions
        solution_1.evaluate();
        solution_2.evaluate();
        println!("SOLUTION 1 OBJECTIVE VALUES: {:?}",solution_1.objective_fitness_values);
        // print pareto dominance values
        let result = ParetoDominance.compare_solutions(&solution_1, &solution_2); 
        println!("LLLL PARETO  {:?}  dddd", result);
        // assert_eq!(pareto_dominance.compare_solutions(&solution_1, &solution_2), -1);
    }

}
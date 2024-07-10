use crate::core::{Problem, Solution};

#[derive(Debug)]
pub struct ParetoDominance<'a> {
    pub solution_1: &'a Solution<'a>, 
    pub solution_2: &'a Solution<'a>
}

impl<'a> ParetoDominance<'a> { 
    pub fn new(solution_1: &'a Solution, solution_2: &'a Solution) -> Self {
        ParetoDominance { solution_1, solution_2 }
        }

    pub fn compare_solutions(&self, solution_1: &Solution, solution_2: &Solution) -> i32 {
            
            let problem: &Problem = &solution_1.problem;
            let range_iter = (0..*problem.number_of_objectives()).into_iter(); 
         
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
            println!("SOLUTION 1 OBJECTIVE VALUES FFFFF: {:?}",solution_1.objective_values);  
            println!("SOLUTION 2 OBJECTIVE VALUES FFFFF: {:?}",solution_2.objective_values);
            for _i in range_iter { 
                println!("INDEX {:?}", _i); 
                let mut obj_1: f64 = solution_1.objective_values[_i]; 
                let mut obj_2 = solution_2.objective_values[_i]; 
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


// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Problem, Solution};
    use crate::gatypes::SolutionType;
    use crate::benchmark_objective_functions::{parabloid_5, parabloid_hyper_5};
    // Create Problem
   
    #[test]
    fn test_pareto_dominance_parablioid_5_dir_pos() {
        // positive direction for single objecteive function
        // R^5 => R^1
        let problem: Problem = Problem {
            solution_length: 3,
            number_of_objectives: 1,
            objective_constraint: Some(vec![Some(10.0)]),
            objective_constraint_operands: Some(vec![Some("<".to_string())]),
            direction: Some(vec![1]),
            solution_data_type: vec![SolutionType::Binary, SolutionType::Integer, SolutionType::Real],
            objective_function: parabloid_5
        };
        let solution_vector1:Vec<f64> = vec![1.0, 2.0, -3.0, 4.0, 5.0];
        let solution_vector2:Vec<f64> = vec![12.0, 10.0, -3.0, 4.0, 5.0];

        let mut solution_1 = Solution {problem: &problem, solution: solution_vector1, 
                                                objective_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_violation: 0, feasible: false, evaluated: false}; 
        let mut solution_2 = Solution {problem: &problem, solution: solution_vector2, 
                                                objective_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_violation: 0, feasible: false, evaluated: false}; 

        // Evaluate Solutions
        solution_1.evaluate();
        solution_2.evaluate();
        let pareto_dominance: ParetoDominance = ParetoDominance::new(&solution_1, &solution_2);
        // print pareto dominance values
        let result = pareto_dominance.compare_solutions(&solution_1, &solution_2); 
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
            solution_data_type: vec![SolutionType::Binary, SolutionType::Integer, SolutionType::Real],
            objective_function: parabloid_5
        };
        let solution_vector1:Vec<f64> = vec![1.0, 2.0, -3.0, 4.0, 5.0];
        let solution_vector2:Vec<f64> = vec![12.0, 10.0, -3.0, 4.0, 5.0];

        let mut solution_1 = Solution {problem: &problem, solution: solution_vector1, 
                                                objective_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_violation: 0, feasible: false, evaluated: false}; 
        let mut solution_2 = Solution {problem: &problem, solution: solution_vector2, 
                                                objective_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_violation: 0, feasible: false, evaluated: false}; 

        // Evaluate Solutions
        solution_1.evaluate();
        solution_2.evaluate();
        let pareto_dominance: ParetoDominance = ParetoDominance::new(&solution_1, &solution_2);
        // print pareto dominance values
        let result = pareto_dominance.compare_solutions(&solution_1, &solution_2); 
        println!("LLLL PARETO  {:?}  dddd", result);

        // assert_eq!(pareto_dominance.compare_solutions(&solution_1, &solution_2), -1);
    }

    #[test]
    fn test_pareto_dominance_parablioid_5_multi_dir_pos() {
        // negative direction for single objecteive function
        // R^5 => R^5
        let problem: Problem = Problem {
            solution_length: 3,
            number_of_objectives: 5,
            objective_constraint: Some(vec![Some(50.0), Some(60.0), Some(70.0), Some(80.0), Some(90.0)]),
            objective_constraint_operands: Some(vec![Some("<".to_string()), Some("<".to_string()), Some("<".to_string()), Some("<".to_string()), Some("<".to_string())], ),
            direction: Some(vec![1, 1, 1, 1, 1]),
            solution_data_type: vec![SolutionType::Binary, SolutionType::Integer, SolutionType::Real, SolutionType::Real, SolutionType::Real],
            objective_function: parabloid_hyper_5
        };
        let solution_vector1:Vec<f64> = vec![1.0, 2.0, -3.0, 4.0, 5.0];
        let solution_vector2:Vec<f64> = vec![12.0, 10.0, -3.0, 4.0, 5.0];

        let mut solution_1 = Solution {problem: &problem, solution: solution_vector1, 
                                                objective_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_violation: 0, feasible: false, evaluated: false}; 
        let mut solution_2 = Solution {problem: &problem, solution: solution_vector2, 
                                                objective_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_violation: 0, feasible: false, evaluated: false}; 

        // Evaluate Solutions
        solution_1.evaluate();
        solution_2.evaluate();
        println!("SOLUTION 1 OBJECTIVE VALUES: {:?}",solution_1.objective_values);
        let pareto_dominance: ParetoDominance = ParetoDominance::new(&solution_1, &solution_2);
        // print pareto dominance values
        let result = pareto_dominance.compare_solutions(&solution_1, &solution_2); 
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
            solution_data_type: vec![SolutionType::Binary, SolutionType::Integer, SolutionType::Real, SolutionType::Real, SolutionType::Real],
            objective_function: parabloid_hyper_5
        };
        let solution_vector1:Vec<f64> = vec![1.0, 2.0, -3.0, 4.0, 5.0];
        let solution_vector2:Vec<f64> = vec![12.0, 10.0, -3.0, 4.0, 5.0];

        let mut solution_1 = Solution {problem: &problem, solution: solution_vector1, 
                                                objective_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_violation: 0, feasible: false, evaluated: false}; 
        let mut solution_2 = Solution {problem: &problem, solution: solution_vector2, 
                                                objective_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_violation: 0, feasible: false, evaluated: false}; 

        // Evaluate Solutions
        solution_1.evaluate();
        solution_2.evaluate();
        println!("SOLUTION 1 OBJECTIVE VALUES: {:?}",solution_1.objective_values);
        let pareto_dominance: ParetoDominance = ParetoDominance::new(&solution_1, &solution_2);
        // print pareto dominance values
        let result = pareto_dominance.compare_solutions(&solution_1, &solution_2); 
        println!("LLLL PARETO  {:?}  dddd", result);

        // assert_eq!(pareto_dominance.compare_solutions(&solution_1, &solution_2), -1);
    }

}
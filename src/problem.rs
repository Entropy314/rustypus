use std::collections::HashMap;

// import SolutionType from types.rs
use types::SolutionType;

pub type CompareFn = fn(i32, i32) -> bool;

// let mut operator_map: HashMap<&'static str, CompareFn> = HashMap::new();
// operator_map.insert("==", |a, b| a == b);
// operator_map.insert("!=", |a, b| a != b);
// operator_map.insert(">", |a, b| a > b);
// operator_map.insert("<", |a, b| a < b);
// operator_map.insert("<=", |a, b| a <= b);
// operator_map.insert(">=", |a, b| a >= b);
// pub type CompareFn = fn(f64, f64) -> bool;


pub struct Problem { 
    // objective function that returns a vector of f64

    // objective_function: fn(solution: &Solution) -> Vec<f64>,
    // constraint is a vector of f64 values
    constraint: Option<Vec<f64>>, 
    constraint_operation: Option<Vec<&'static str>>
    constraint_violation: Option<Vec<bool>>, 
    optimizing: Option<Vec<i64>>
    // constraint function that returns a vector of f64

}

// checck that constraint and constraint violation are the same lendgth
// check that constraint and constraint operation are the same length
impl Problem {
    fn new( objective_function: fn(solution: &Solution) -> Vec<f64>, constraint: Option<Vec<f64>>, constraint_operation: Option<Vec<&'static str>>, constraint_violation: Option<Vec<bool>>) -> Self {
        // Optimizing by default is minimizing
        let optimizing = match minimize {
            1 => Some(vec![1]),
            _ => Some(vec![-1])
        };
        // check if constraint and constraint and optimizing and  constraint_violation are the same length and panic if not
        if constraint.is_some() && constraint_operation.is_some() && constraint_violation.is_some() && optimizing.is_some() {
            if constraint.unwrap().len() != constraint_operation.unwrap().len() || constraint.unwrap().len() != constraint_violation.unwrap().len() || constraint.unwrap().len() != optimizing.unwrap().len() {
                panic!("Constraint, constraint operation, constraint violation, and optimizing must be the same length");
            }
        }
        
        Problem { minimize, maximize, objective_function, constraint, constraint_operation, constraint_violation, optimizing }
    }
}

// Create Unit Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem() {
        let problem = Problem::new(1, 0, |solution| vec![solution.fitness[0]], None, None, None);
        assert!(problem.minimize == 1);
        assert!(problem.maximize == 0);
        assert!(problem.constraint.is_none());
        assert!(problem.constraint_operation.is_none());
        assert!(problem.constraint_violation.is_none());
        assert!(problem.optimizing.is_some());
        assert!(problem.optimizing.unwrap()[0] == 1);
    }

    #[test]
    fn test_problem_with_constraint() {
        let problem = Problem::new(1, 0, |solution| vec![solution.fitness[0]], Some(vec![1.0]), Some(vec![">="]), Some(vec![true]));
        assert!(problem.minimize == 1);
        assert!(problem.maximize == 0);
        assert!(problem.constraint.is_some());
        assert!(problem.constraint_operation.is_some());
        assert!(problem.constraint_violation.is_some());
        assert!(problem.optimizing.is_some());
        assert!(problem.optimizing.unwrap()[0] == 1);
    }

    #[test]
    #[should_panic]
    fn test_problem_with_constraint_panic() {
        let problem = Problem::new(1, 0, |solution| vec![solution.fitness[0]], Some(vec![1.0]), Some(vec![">="]), Some(vec![true, false]));
    }
}

// pub struct Solution { 
//     solution_types: Vec<SolutionType>,
//     is_feasible: bool,
//     is_evaluated: bool,
//     fitness: Vec<f64>, 

//     objective: Vec<f64>, 
//     problem: Problem
// }

// // Implement a method in SOlution where it creates the Vec of values from SolutionType
// impl Solution {
//     fn new(solution_types: Vec<SolutionType>) -> Self {
//         let mut solution = Solution {
//             solution_types,
//             is_feasible: false,
//             is_evaluated: false,
//             fitness: Vec::new(),
//             constraint_violation: Vec::new(),
//             objective: Vec::new(),
//         };
//         solution.generate_solution();
//         solution
//     }

//     fn generate_solution(&mut self) {
//         for solution_type in &self.solution_types {
//             match solution_type {
//                 SolutionType::Binary => {
//                     let binary = Binary::new();
//                     self.fitness.push(binary.value() as f64);
//                 },
//                 SolutionType::Integer => {
//                     let integer = Integer::new(None, None);
//                     self.fitness.push(integer.value() as f64);
//                 },
//                 SolutionType::Real => {
//                     let real = Real::new(None, None);
//                     self.fitness.push(real.value());
//                 }
//             }
//         }
//     }
// }
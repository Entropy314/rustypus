// import SolutionTypes, Binary, Integer, Real from  gatypes.rs
use crate::gatypes::{SolutionType, Binary, Integer, Real};

pub struct Problem {
    solution_length: usize,
    number_of_objectives: usize,
    // Upper or Lower bound for the objective function
    objective_constraint: Option<Vec<f64>>, // [10, 20]
    // Operands for Greater than or less than the objective constraint
    objective_constraint_operands: Option<Vec<String>>, // ["<", ">"]
    // Defaults vector to -1 with length of number_of_objectives
    optimizing_objective_vector: Option<Vec<i8>>, // [-1, -1]
    // solution type is a vector of the solution types
    solution_type_vector: Vec<SolutionType>, // [Binary, Integer(lower_bound:Some(10), upper_bound:Some(20)), Real(lower_bound:Some(1.0), upper_bound:Some(20.0))]
    // Objective function that takes the SolutionTypes vector values and returns a vector of f64 values
    objective_function: fn(solution: &Vec<f64>) -> f64,
}

impl Problem {
    pub fn new(
        solution_length: usize,
        number_of_objectives: usize,
        objective_constraint: Option<Vec<f64>>,
        objective_constraint_operands: Option<Vec<String>>,
        optimizing_objective_vector: Option<Vec<i8>>,
        solution_type_vector: Vec<SolutionType>,
        objective_function: fn(&Vec<f64>) -> f64,
    ) -> Self {
        Problem {
            solution_length,
            number_of_objectives,
            objective_constraint,
            objective_constraint_operands,
            optimizing_objective_vector,
            solution_type_vector,
            objective_function,
        }
    }

    pub fn solution_length(&self) -> &usize {
        &self.solution_length
    }

    pub fn number_of_objectives(&self) -> &usize {
        &self.number_of_objectives
    }

    pub fn objective_constraint(&self) -> &Option<Vec<f64>> {
        &self.objective_constraint
    }

    pub fn objective_constraint_operands(&self) -> &Option<Vec<String>> {
        // Check if Operatnds are are <, >, <=, >=, ==, !=
        let operands = &self.objective_constraint_operands;
        if operands.is_some() {
            let operands = operands.as_ref().unwrap();
            for operand in operands {
                if operand != "<" && operand != ">" && operand != "<=" && operand != ">=" && operand != "==" && operand != "!=" {
                    panic!("Invalid operand: {}", operand);
                }
            }
        }
        
        &self.objective_constraint_operands
    }

    pub fn optimizing_objective_vector(&self) -> &Option<Vec<i8>> {
        &self.optimizing_objective_vector
    }

    pub fn solution_type_vector(&self) -> &Vec<SolutionType> {
        &self.solution_type_vector
    }

    pub fn objective_function(&self) -> &fn(&Vec<f64>) -> f64 {
        &self.objective_function
    }

    pub fn generate_solution(&self) -> Vec<f64> {
        let mut solution: Vec<f64> = Vec::new();
        let real_lower_bound = 10.0;
        let real_upper_bound: f64 = 20.0;
        let integer_lower_bound: i64 = 10;
        let integer_upper_bound: i64 = 20;
        for solution_type in &self.solution_type_vector {
            match solution_type {
                SolutionType::Binary => {
                    let binary = Binary::new();
                    solution.push(binary.value() as f64);
                }
                SolutionType::Integer => {
                    let integer = Integer::new(Some(integer_lower_bound), Some(integer_upper_bound));
                    solution.push(integer.value() as f64);
                }
                SolutionType::Real => {
                    let real = Real::new(Some(real_lower_bound), Some(real_upper_bound));
                    solution.push(real.value());
                }
            }
        }
        println!("Solution: {:?}", solution);
        solution
    }
}

// Create Unit Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem() {
        let problem = Problem::new(10, 2, 
                                            Some(vec![10.0, 20.0]), Some(vec![">".to_string(), "<".to_string()]), 
                                            Some(vec![-1, -1]), 
                                            vec![SolutionType::Binary, SolutionType::Integer, SolutionType::Real], 
                                            |solution: &Vec<f64>| -> f64 { 0.0 });
        assert!(problem.solution_length() == &10);
        assert!(problem.number_of_objectives() == &2);
        assert!(matches!(problem.objective_constraint(), Some(vec) if vec == &[10.0, 20.0]));
        assert!(matches!(problem.objective_constraint_operands(), Some(vec) if vec == &[">".to_string(), "<".to_string()]));
        assert!(matches!(problem.optimizing_objective_vector(), Some(vec) if vec == &[-1, -1]));
        assert!(problem.solution_type_vector() == &vec![SolutionType::Binary, SolutionType::Integer, SolutionType::Real]);
        assert!(problem.objective_function()(&vec![0.0 as f64,0.0 as f64,0.0 as f64]) == 0.0);
        // Solution Vector
        let solution = problem.generate_solution();
        assert!(solution.len() == 3);
    }
    // Anothher Unit test with different values objective_constraint, objective_constraint_operands, optimizing_objective_vector
    #[test] // All variables all constrants with objective function x**2 + y**2  + z**2
    fn test_problem2() {
        let problem = Problem::new(10, 2, 
                                            Some(vec![10.0, 20.0]), Some(vec![">".to_string(), "<".to_string()]), 
                                            Some(vec![-1, -1]), 
                                            vec![SolutionType::Binary, SolutionType::Integer, SolutionType::Real], 
                                            |solution: &Vec<f64>| -> f64 { solution[0].powf(2.0) + solution[1].powf(2.0) + solution[2].powf(2.0) });
        assert!(problem.solution_length() == &10);
        assert!(problem.number_of_objectives() == &2);
        assert!(matches!(problem.objective_constraint(), Some(vec) if vec == &[10.0, 20.0]));
        assert!(matches!(problem.objective_constraint_operands(), Some(vec) if vec == &[">".to_string(), "<".to_string()]));
        assert!(matches!(problem.optimizing_objective_vector(), Some(vec) if vec == &[-1, -1]));
        assert!(problem.solution_type_vector() == &vec![SolutionType::Binary, SolutionType::Integer, SolutionType::Real]);
        assert!(problem.objective_function()(&vec![0.0 as f64,0.0 as f64,0.0 as f64]) == 0.0);
        // Solution Vector
        let solution = problem.generate_solution();
        assert!(solution.len() == 3);
    }
    
    #[test] // Solution length is 100 parameters long Each element in solution vector alternates with Binary, Integer, Real and randomize the constraints
    fn test_problem3() {
        let mut solution_type_vector: Vec<SolutionType> = Vec::new();
        let mut objective_constraint: Vec<f64> = Vec::new();
        let mut objective_constraint_operands: Vec<String> = Vec::new();
        let mut optimizing_objective_vector: Vec<i8> = Vec::new();
        for i in 0..100 {
            if i % 3 == 0 {
                solution_type_vector.push(SolutionType::Binary);
                objective_constraint.push(0.0);
                objective_constraint_operands.push("".to_string());
                optimizing_objective_vector.push(0);
            } else if i % 3 == 1 {
                solution_type_vector.push(SolutionType::Integer);
                objective_constraint.push(0.0);
                objective_constraint_operands.push("".to_string());
                optimizing_objective_vector.push(0);
            } else {
                solution_type_vector.push(SolutionType::Real);
                objective_constraint.push(0.0);
                objective_constraint_operands.push("".to_string());
                optimizing_objective_vector.push(0);
            }
        }
        let problem = Problem::new(100, 2, 
                                            Some(objective_constraint), Some(objective_constraint_operands), 
                                            Some(optimizing_objective_vector), 
                                            solution_type_vector, 
                                            |solution: &Vec<f64>| -> f64 { 0.0 });
        assert!(problem.solution_length() == &100);
        assert!(problem.number_of_objectives() == &2);
        assert!(matches!(problem.objective_constraint(), Some(vec) if vec == &[0.0; 100]));
        // assert!(matches!(problem.objective_constraint_operands(), Some(vec) if vec == &["".to_string(); 100]));
        assert!(matches!(problem.optimizing_objective_vector(), Some(vec) if vec == &[0; 100]));
        assert!(problem.solution_type_vector().len() == 100);
        assert!(problem.objective_function()(&vec![0.0 as f64,0.0 as f64,0.0 as f64]) == 0.0);
        // Solution Vector
        let solution = problem.generate_solution();
        assert!(solution.len() == 100);
    }

}
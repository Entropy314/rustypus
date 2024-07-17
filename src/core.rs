// import SolutionTypes, BitBinary, Integer, Real from  gatypes.rs
// use crate::gatypes::{SolutionType, BitBinary, Integer, Real};
use crate::constraints::ComparisonFunctions;

use crate::gatypes::SolutionDataTypes;
use crate::gatypes::{BitBinary, Integer, Real};
#[derive(Debug)]
pub struct Problem {
    pub solution_length: usize,
    pub number_of_objectives: usize,
    pub objective_constraint: Option<Vec<Option<f64>>>, // Upper or Lower bound for the objective function eg. [10, 20]
    pub objective_constraint_operands: Option<Vec<Option<String>>>, // Operands for Greater than or less than the objective constraint eg. ["<", ">"]
    pub direction: Option<Vec<i8>>, // Defaults vector to -1 with length of number_of_objectives eg. [-1, -1]
    pub solution_data_types: Vec<SolutionDataTypes>,     // solution type is a vector of the solution types eg. [BitBinary, Integer(lower_bound:Some(10), upper_bound:Some(20)), Real(lower_bound:Some(1.0), upper_bound:Some(20.0))]
    pub objective_function: fn(solution: &Vec<f64>) -> Vec<f64> // Objective function that takes the SolutionTypes vector values and returns a vector of f64 values
}

impl Problem {
    pub fn new(
        solution_length: usize,
        number_of_objectives: usize,
        objective_constraint: Option<Vec<Option<f64>>>, //number_of_objectives
        objective_constraint_operands: Option<Vec<Option<String>>>, //number_of_objectives
        direction: Option<Vec<i8>>,
        solution_data_types: Vec<SolutionDataTypes>,// Vec of Binary or Integer or Real
        objective_function: fn(&Vec<f64>) -> Vec<f64>
    ) -> Self {
        // If solution_length != solution_data_types.len() panic
        if solution_length != solution_data_types.len() {
            panic!("solution_length does not match solution_data_types length");
        }

        // Check if lengths match number_of_objectives
        if let Some(ref constraints) = objective_constraint {
            if constraints.len() != number_of_objectives {
                panic!("objective_constraint length does not match number_of_objectives");
            }
        }

        if let Some(ref operands) = objective_constraint_operands {
            if operands.len() != number_of_objectives {
                panic!("objective_constraint_operands length does not match number_of_objectives");
            }
        }

        let direction: Option<Vec<i8>> = direction.or_else(|| Some(vec![-1; number_of_objectives]));

        if let Some(ref dirs) = direction {
            if dirs.len() != number_of_objectives {
                panic!("direction length does not match number_of_objectives");
            }
        }

        Problem {
            solution_length,
            number_of_objectives,
            objective_constraint,
            objective_constraint_operands,
            direction,
            solution_data_types,
            objective_function
        }
    }

    pub fn solution_length(&self) -> &usize {
        &self.solution_length
    }

    pub fn number_of_objectives(&self) -> &usize {
        &self.number_of_objectives
    }

    pub fn objective_constraint(&self) -> &Option<Vec<Option<f64>>> {
        &self.objective_constraint
    }

    pub fn objective_constraint_operands(&self) -> &Option<Vec<Option<String>>> {
        // Check if Operatnds are are <, >, <=, >=, ==, !=
        let operands = &self.objective_constraint_operands;
        if operands.is_some() {
            let operands = operands.as_ref().unwrap();
            for operand in operands {
                let operand = operand.as_ref().unwrap();
                if operand != "<" && operand != ">" && operand != "<=" && operand != ">=" && operand != "==" && operand != "!=" {
                    panic!("Invalid operand: {}", operand);
                }
            }
        }
        
        &self.objective_constraint_operands
    }

    pub fn direction(&self) -> &Option<Vec<i8>> {
        &self.direction
    }

    pub fn solution_data_type(&self) -> &Vec<SolutionDataTypes> {
        &self.solution_data_types
    }

    pub fn objective_function(&self) -> &fn(&Vec<f64>) -> Vec<f64> {
        &self.objective_function
    }

    pub fn generate_solution(&self) -> Vec<f64> {
        let mut solution: Vec<f64> = Vec::new();
        let real_lower_bound = 10.0;
        let real_upper_bound: f64 = 20.0;
        let integer_lower_bound: i64 = 10;
        let integer_upper_bound: i64 = 20;
        for solution_type in &self.solution_data_types {
            match solution_type {
                SolutionDataTypes::BitBinary(ref b) => {
                    let binary: BitBinary = BitBinary::new(None);
                    solution.push(binary.value().unwrap() as f64);
                }
                SolutionDataTypes::Integer(ref i) => {
                    let integer: Integer = Integer::new(None, Some(integer_lower_bound), Some(integer_upper_bound));
                    solution.push(integer.value().unwrap() as f64);
                }
                SolutionDataTypes::Real(ref r) => {
                    let real = Real::new(None, Some(real_lower_bound), Some(real_upper_bound));
                    solution.push(real.value().unwrap());
                }
            }
        }
        solution
    }
}

#[derive(Debug)]
pub struct Solution<'a> { 
    pub problem: &'a Problem,
    pub solution: Vec<f64>, // Derived from Problem.solution_data_types
    pub objective_fitness_values: Vec<f64>,
    pub constraint_values: Vec<f64>,
    pub evaluated: bool, // default false
    pub constraint_violation: usize, // default 0
    pub feasible: bool
}


impl<'a> Solution<'a> { 
    pub fn new(problem: &'a Problem) -> Self {
        let solution = problem.generate_solution();
        // create vectore of length number_of_objectives
        let objective_fitness_values: Vec<f64> = Vec::with_capacity(*problem.number_of_objectives());
        let constraint_values: Vec<f64> = Vec::with_capacity(*problem.number_of_objectives());
        let evaluated = false;
        let constraint_violation = 0;
        let feasible = false;

        Solution {
            problem,
            solution,
            objective_fitness_values,
            constraint_values,
            evaluated,
            constraint_violation,
            feasible,
        }
    }

    pub fn problem(&self) -> &Problem {
        &self.problem
    }

    pub fn solution(&self) -> &Vec<f64> {
        &self.solution
    }

    pub fn objective_fitness_values(&self) -> &Vec<f64> {
        &self.objective_fitness_values
    }

    pub fn constraint_values(&self) -> &Vec<f64> {
        &self.constraint_values
    }

    pub fn evaluated(&self) -> &bool {
        &self.evaluated
    }

    pub fn constraint_violation(&self) -> &usize {
        &self.constraint_violation
    }

    pub fn feasible(&self) -> &bool {
        &self.feasible
    }

    pub fn evaluate_constraints(&mut self) -> Vec<f64> {
        let mut constraint_values: Vec<f64> = Vec::new();
        let objective_constraint = self.problem.objective_constraint();
        let objective_constraint_operands = self.problem.objective_constraint_operands();
        // println!("OBJECTIVE CONSTRAINT: {:?}", objective_constraint);
        // println!("OBJECTIVE CONSTRAINT OPERANDS: {:?}", objective_constraint_operands);
        if objective_constraint.is_some() && objective_constraint_operands.is_some() {
            let objective_constraint: &Vec<Option<f64>> = &objective_constraint.as_ref().unwrap();
            let objective_constraint_operands: &Vec<Option<String>> = &objective_constraint_operands.as_ref().unwrap();
            for _i in 0..objective_constraint.len() {
                let operand: &Option<String> = &objective_constraint_operands[_i];
                let constraint: &Option<f64> = &objective_constraint[_i];
                let obj_value: &f64 = &self.objective_fitness_values[_i];
                // println!("OBJ VALUE: {:?}", obj_value);
                let comparison_fn= ComparisonFunctions::new();// operand.as_ref().unwrap();
                // println!("OPERAND: {:?}", operand);
                let comparison_fn = comparison_fn.functions.get(operand.as_ref().unwrap()).unwrap();
                let constraint_value = comparison_fn.compare(*obj_value, constraint.unwrap());
                // println!("CONSTRAINT VALUE: {:?}", constraint_value);
                constraint_values.push(constraint_value as i8 as f64);
            }
        }
        constraint_values
    }

    pub fn calculate_constraint_violation(&mut self) -> usize {
        let mut constraint_violation = 0;
        let constraint_values: &Vec<f64> = self.constraint_values();
        for constraint_value in constraint_values {
            if *constraint_value == 0.0 {
                constraint_violation += 1;
            }
        }
        constraint_violation
    }

    pub fn is_feasible(&mut self) -> bool {
        let constraint_violation = self.constraint_violation();
        if *constraint_violation == 0 {
            true
        } else {
            false
        }
    }

    pub fn evaluate(&mut self) {
        let objective_fitness_values = (self.problem.objective_function)(&self.solution);
        self.evaluated = true;
        self.objective_fitness_values = objective_fitness_values;
        self.feasible = self.is_feasible();
        // self.constraint_violation = self.calculate_constraint_violation();
        // self.constraint_values = self.evaluate_constraints();

    }

}



// Write Unit Tests

#[cfg(test)]
mod tests {
    use std::vec;
    use crate::gatypes::{SolutionDataTypes, BitBinary, Integer, Real};
    // import functions from benchmark_objective_functions.rs
    use crate::core::{Problem, Solution};
    use crate::benchmark_objective_functions::{simple_objective, dtlz1, dtlz2, dtlz3, dtlz4, dtlz5, dtlz6, dtlz7, xyz_objective};
    #[test]
    fn test() { 
        let a: usize = 20; 
        let b = &a; 
        for _i in 0..*b { 
            println!("Hello World: {:?}", _i);
        }
        // Create BitBinary and Real Solution Types
        let binary = BitBinary::new(None);
        let integer: Integer = Integer::new(None, Some(10), Some(20));
        let real = Real::new(None, Some(10.0), Some(20.0));
        println!("{:?}", integer);
        println!("{:?}", binary);
        println!("{:?}", real);


    }

    #[test]
    fn test_problem() {
        let problem = Problem::new(
            3,
            2,
            Some(vec![Some(10.0), Some(20.0)]),
            Some(vec![Some(">".to_string()), Some("<".to_string())]),
            Some(vec![-1, -1]),
            vec![SolutionDataTypes::new_binary(None) , SolutionDataTypes::new_integer( None, Some(10), Some(20)) , SolutionDataTypes::new_real(None, Some(10.0), Some(20.0))], 
            |solution: &Vec<f64>| {
                let mut objective_fitness_values: Vec<f64> = Vec::new();
                objective_fitness_values.push(solution[0] + solution[1]);
                objective_fitness_values.push(solution[2]);
                objective_fitness_values
            }
        );
        assert_eq!(*problem.solution_length(), 3);
        assert_eq!(*problem.number_of_objectives(), 2);
        assert_eq!(*problem.objective_constraint(), Some(vec![Some(10.0), Some(20.0)]));
        assert_eq!(*problem.objective_constraint_operands(), Some(vec![Some(">".to_string()), Some("<".to_string())]));
        assert_eq!(*problem.direction(), Some(vec![-1, -1]));
        // assert_eq!(*problem.solution_data_types(), vec![SolutionType::BitBinary, SolutionType::Integer, SolutionType::Real]);
        assert_eq!(*problem.objective_function()(&vec![1.0, 2.0, 3.0]), vec![3.0, 3.0]);
    }

    #[test]
    fn test_types_are_good() { 
        let bin = BitBinary::new(None);
        let int: Integer = Integer::new(None, Some(10), Some(20));
        let real: Real = Real::new(None, Some(10.0), Some(20.0));
        println!("{:?}", bin.value());
        println!("{:?}", int.value());
        println!("{:?}", real.value());
        

    }

    #[test]
    fn test_problem_generate_solution() {
        let problem: Problem = Problem::new(
            3,
            2,
            Some(vec![Some(10.0), Some(20.0)]),
            Some(vec![Some(">".to_string()), Some("<".to_string())]),
            Some(vec![-1, -1]),
            vec![SolutionDataTypes::new_binary(None) , SolutionDataTypes::new_integer( None, Some(10), Some(20)) , SolutionDataTypes::new_real(None, Some(10.0), Some(20.0))], 
            |solution: &Vec<f64>| {
                let mut objective_fitness_values: Vec<f64> = Vec::new();
                objective_fitness_values.push(solution[0] + solution[1]);
                objective_fitness_values.push(solution[2]);
                objective_fitness_values
            }
        );
        let solution_vector = problem.generate_solution();
        println!("SOLUTION VECTOR: {:?}", solution_vector);
        assert_eq!(solution_vector.len(), 3);
    }

    #[test] // Test Solution
    fn test_solution_no_constraints() {
        // TODO: FIX TEST objective function still outputs and empty vector

        let problem: Problem = Problem {
            solution_length: 3,
            number_of_objectives: 2,
            objective_constraint: Some(vec![Some(10.0), Some(20.0)]),
            objective_constraint_operands: Some(vec![Some(">".to_string()), Some("<".to_string())]),
            direction: Some(vec![-1, -1]),
            solution_data_types: vec![SolutionDataTypes::new_binary(None) , SolutionDataTypes::new_integer( None, Some(10), Some(20)) , SolutionDataTypes::new_real(None, Some(10.0), Some(20.0))], 
            objective_function: xyz_objective,
        };
        
        let mut solution: Solution<'_> = Solution::new(&problem);
        solution.solution = solution.problem.generate_solution();
        solution.evaluate();
        
        assert_eq!(solution.evaluated, true);
        assert_eq!(solution.solution.len(), 3);
        assert_eq!(solution.objective_fitness_values().len(), 2);
        assert_eq!(solution.constraint_values().len(), 0);
    }
    #[test]
    fn test_solution_with_constraints_() { 

        
        let problem: Problem = Problem {
            solution_length: 3,
            number_of_objectives: 2,
            objective_constraint: Some(vec![Some(10.0), Some(20.0)]),
            objective_constraint_operands: Some(vec![Some(">".to_string()), Some("<".to_string())]),
            direction: Some(vec![-1, -1]),
            solution_data_types: vec![SolutionDataTypes::new_binary(None) , SolutionDataTypes::new_integer( None, Some(10), Some(20)) , SolutionDataTypes::new_real(None, Some(10.0), Some(20.0))], 
            objective_function: simple_objective,
        };
        
        let mut solution: Solution<'_> = Solution::new(&problem);
        solution.solution = vec![1.0, 2.0, 3.0]; 
        solution.evaluate();
       
        assert_eq!(vec![14.0, 8.0], solution.objective_fitness_values);
        assert_eq!(true, solution.is_feasible());
        assert_eq!(0, solution.calculate_constraint_violation());        
    }

    #[test]
    fn test_solution_with_constraints_dtlz1() { 
        // let vectr = vec![1.0, 2.0, 3.0, -4.0, -5.0];
       
        let problem: Problem = Problem {
            solution_length: 3,
            number_of_objectives: 2,
            objective_constraint: Some(vec![Some(10.0), Some(20.0)]),
            objective_constraint_operands: Some(vec![Some(">".to_string()), Some("<".to_string())]),
            direction: Some(vec![-1, -1]),
            solution_data_types: vec![SolutionDataTypes::new_binary(None) , SolutionDataTypes::new_integer( None, Some(10), Some(20)) , SolutionDataTypes::new_real(None, Some(10.0), Some(20.0))], 
            objective_function: dtlz1
        };
        
        let mut solution: Solution<'_> = Solution::new(&problem);
        solution.solution = vec![1.0, 2.0, 3.0, -4.0, -5.0]; 
        solution.evaluate();
        
        assert_eq!(vec![-12.0, 3.0, 1.0, 0.5], solution.objective_fitness_values);
        assert_eq!(true, solution.is_feasible());
        assert_eq!(0, solution.calculate_constraint_violation());        
    }
    // TODO: Test Panic Cases

    #[test]
    fn test_view_solution() { 
        let problem: Problem = Problem {
            solution_length: 3,
            number_of_objectives: 2,
            objective_constraint: Some(vec![Some(10.0), Some(20.0)]),
            objective_constraint_operands: Some(vec![Some(">".to_string()), Some("<".to_string())]),
            direction: Some(vec![-1, -1]),
            solution_data_types: vec![SolutionDataTypes::new_binary(None) , SolutionDataTypes::new_integer( None, Some(10), Some(20)) , SolutionDataTypes::new_real(None, Some(10.0), Some(20.0))], 
            objective_function: dtlz1
        };
        
        let mut solution: Solution<'_> = Solution::new(&problem);
        solution.solution = vec![1.0, 2.0, 3.0, -4.0, -5.0];
        println!("SOLUTION: {:?}", solution.solution);
        println!("OBJECTIVE VALUES: {:?}", solution.problem.solution_data_types);
    }

}
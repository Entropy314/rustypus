// import SolutionTypes, Binary, Integer, Real from  gatypes.rs
use crate::gatypes::{SolutionType, Binary, Integer, Real};
use crate::{constraints::ComparisonFunctions};


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
    objective_function: fn(solution: &Vec<f64>) -> Vec<f64>,
}

impl Problem {
    pub fn new(
        solution_length: usize,
        number_of_objectives: usize,
        objective_constraint: Option<Vec<f64>>, //number_of_objectives
        objective_constraint_operands: Option<Vec<String>>, //number_of_objectives
        optimizing_objective_vector: Option<Vec<i8>>,
        solution_type_vector: Vec<SolutionType>,
        objective_function: fn(&Vec<f64>) -> Vec<f64>
    ) -> Self {
        Problem {
            solution_length,
            number_of_objectives,
            objective_constraint,
            objective_constraint_operands,
            optimizing_objective_vector,
            solution_type_vector,
            objective_function
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

    pub fn objective_function(&self) -> &fn(&Vec<f64>) -> Vec<f64> {
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



pub struct Solution<'a> { 
    problem: &'a Problem,
    solution: Vec<f64>, // Derived from Problem.solution_type_vector
    objective_values: Vec<f64>,
    constraint_values: Vec<f64>,
    evaluated: bool, // default false
    constraint_violation: usize, // default 0
    feasible: bool
}


impl<'a> Solution<'a> { 
    pub fn new(problem: &'a Problem) -> Self {
        let solution = problem.generate_solution();
        // create vectore of length number_of_objectives
        let objective_values: Vec<f64> = Vec::with_capacity(*problem.number_of_objectives());
        let constraint_values: Vec<f64> = Vec::with_capacity(*problem.number_of_objectives());
        let evaluated = false;
        let constraint_violation = 0;
        let feasible = false;
        Solution {
            problem,
            solution,
            objective_values,
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

    pub fn objective_values(&self) -> &Vec<f64> {
        &self.objective_values
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
        println!("AAAAAAAAAAAAAAAAAAAAAAAAA");
        let objective_constraint = self.problem.objective_constraint();
        println!("BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB");
        let objective_constraint_operands = self.problem.objective_constraint_operands();
        println!("CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC");
        println!("objective_constraint: {:?}", objective_constraint);
        println!("objective_constraint_operands: {:?}", objective_constraint_operands);
        if objective_constraint.is_some() && objective_constraint_operands.is_some() {
            let objective_constraint = objective_constraint.as_ref().unwrap();
            println!("FFFFFFFFFFFFFFFFFF");
            println!("objective constrant length: {}", objective_constraint.len());
            let objective_constraint_operands: &Vec<String> = objective_constraint_operands.as_ref().unwrap();
            println!("GGGGGGGGGGGGGGGGGGGGG: {}", objective_constraint_operands.len()); 
            for i in 0..objective_constraint.len() {
                println!("INDEXXXXXXXXXXX: {}", i);
                let operand = &objective_constraint_operands[i];
                println!("OPERAND: P{:?}", operand);
                let constraint = &objective_constraint[i];
                println!("CONSTRAINT: {:?}", constraint);
                println!("OBJECTIVE VALUESSSS: {:?}", self.objective_values);
                let objective_value = self.objective_values[i];
                println!("OBJECTIVE VALUE: {:?}", objective_value);
                let comparison_fn = ComparisonFunctions::new();
                let comparison_fn = comparison_fn.functions.get(operand).unwrap();
                let constraint_value = comparison_fn.compare(objective_value, *constraint);
                constraint_values.push(constraint_value as i8 as f64);
            }
        }
        constraint_values
    }

    pub fn calculate_constraint_violation(&mut self) -> usize {
        let mut constraint_violation = 0;
        let constraint_values = self.constraint_values();
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
        println!("SOLUTION: {:?}", &self.solution);
        let objective_values = (self.problem.objective_function)(&self.solution);
        println!("OBJECTIVE VALUES: {:?}", objective_values);
        println!("DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD{:?}   ", objective_values.len());
        self.objective_values = objective_values;

    // pub evaluate_constraints(&mut self) -> Vec<f64> {

    //     let constraint_values = self.evaluate_constraints();
    //     println!("OBJECTIVE VALUES: {:?}", objective_values);
    //     let evaluated = true;
    //     let constraint_violation = self.calculate_constraint_violation();
    //     let feasible = self.is_feasible();
    //     println!("SELF OBJECTIVE VALUES: {:?}", self.objective_values); 
    //     self.constraint_values = constraint_values;
    //     self.evaluated = evaluated;
    //     self.constraint_violation = constraint_violation;
    //     self.feasible = feasible;
    // }
    }

}


// Write Unit Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem() {
        let problem = Problem::new(
            3,
            2,
            Some(vec![10.0, 20.0]),
            Some(vec![">".to_string(), "<".to_string()]),
            Some(vec![-1, -1]),
            vec![SolutionType::Binary, SolutionType::Integer, SolutionType::Real],
            |solution: &Vec<f64>| {
                let mut objective_values: Vec<f64> = Vec::new();
                objective_values.push(solution[0] + solution[1]);
                objective_values.push(solution[2]);
                objective_values
            }
        );
        assert_eq!(*problem.solution_length(), 3);
        assert_eq!(*problem.number_of_objectives(), 2);
        assert_eq!(*problem.objective_constraint(), Some(vec![10.0, 20.0]));
        assert_eq!(*problem.objective_constraint_operands(), Some(vec![">".to_string(), "<".to_string()]));
        assert_eq!(*problem.optimizing_objective_vector(), Some(vec![-1, -1]));
        // assert_eq!(*problem.solution_type_vector(), vec![SolutionType::Binary, SolutionType::Integer, SolutionType::Real]);
        assert_eq!(*problem.objective_function()(&vec![1.0, 2.0, 3.0]), vec![3.0, 3.0]);
    }

    #[test]
    fn test_generate_solution() {
        let problem = Problem::new(
            3,
            2,
            Some(vec![10.0, 20.0]),
            Some(vec![">".to_string(), "<".to_string()]),
            Some(vec![-1, -1]),
            vec![SolutionType::Binary, SolutionType::Integer, SolutionType::Real],
            |solution: &Vec<f64>| {
                let mut objective_values: Vec<f64> = Vec::new();
                objective_values.push(solution[0] + solution[1]);
                objective_values.push(solution[2]);
                objective_values
            }
        );
        let solution = problem.generate_solution();
        assert_eq!(solution.len(), 3);
    }

    #[test] // Test Solution
    fn test_solution() {
        // TODO: FIX TEST objective function still outputs and empty vector
        
        let problem: Problem = Problem::new(
            3,
            2,
            Some(vec![10.0, 20.0]),
            Some(vec![">".to_string(), "<".to_string()]),
            Some(vec![-1, -1]),
            vec![SolutionType::Binary, SolutionType::Integer, SolutionType::Real],
            |solution: &Vec<f64>| {
                let mut objective_values: Vec<f64> = Vec::new();
                println!("VECTOR: {:?}", objective_values);
                objective_values.push(solution[0] + solution[1]);
                println!("VECTOR: {:?}", objective_values);

                objective_values.push(solution[2]);
                println!("VECTOR: {:?}", objective_values);

                objective_values
            }
            
        );
        let solution: Solution<'_> = Solution::new(&problem);
        assert_eq!(solution.solution().len(), 3);
        assert_eq!(solution.objective_values().len(), 0);
        assert_eq!(solution.constraint_values().len(), 0);
        assert_eq!(*solution.evaluated(), false);
        assert_eq!(*solution.constraint_violation(), 0);
        assert_eq!(*solution.feasible(), false);
        assert_eq!(solution.objective_values().len(), 2);
        // assert_eq!(solution.constraint_values().len(), 2);
        // assert_eq!(*solution.evaluated(), true);
        // assert_eq!(*solution.constraint_violation(), 2);
        // assert_eq!(*solution.feasible(), false);
    }

}
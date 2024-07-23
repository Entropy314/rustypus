use crate::dominance::ParetoDominance;
use crate::core::Solution; 
use crate::dominance::Dominance;
use rand::Rng;

pub trait Selector { 
    fn select<'a>(&self, population: &[Solution], n: usize) -> Vec<Solution>;
    fn select_one<'a>(&self, population: &Vec<&'a Solution<'_>>) -> &Solution;
}

#[derive(Debug)]
pub struct TournamentSelector {
    tournament_size: i64,
    dominance:  ParetoDominance
}


// Default tournament size is 2 and default dominance is ParetoDominance
impl Default for TournamentSelector {
    fn default() -> Self {
        TournamentSelector {
            tournament_size: 2,
            dominance: ParetoDominance
        }
    }
}

impl TournamentSelector {
    pub fn select_one<'a>(&'a self, population: &Vec<&'a Solution<'_>>) -> &Solution {
        // select a random solution from the population
        let mut rng = rand::thread_rng();
        let mut winner = population[rng.gen_range(0..population.len())];

        for _i in 0..self.tournament_size {
            let challenger = population[rng.gen_range(0..population.len())];
            let flag: i32 = self.dominance.compare_solutions(&challenger, &winner);
            if flag > 0 { 
                winner = challenger
            }
        }
        winner
        
    }

    pub fn select<'a>(&'a self, n: usize, population: &'a Vec<&'a Solution<'a>>) -> Vec<&'a Solution<'_>> {
        (0..n).map(|_| self.select_one(population)).collect()
    }

}


// Unit tests for TournamentSelector
#[cfg(test)]
mod tests {
    use super::*;
    use std::mem; 
    use crate::core::{Solution, Problem};
    use crate::gatypes::{SolutionDataTypes, BitBinary, Integer, Real};
    use crate::benchmark_objective_functions::{parabloid_5_loc, parabloid_hyper_5};
    // use crate::dominance::DominanceEnum::ParetoDominance;

    

    #[test]
    fn test_tournament_selector_single_objective() {
     
        let problem: Problem = Problem {
            solution_length: 3,
            number_of_objectives: 1,
            objective_constraint: Some(vec![Some(10.0)]),
            objective_constraint_operands: Some(vec![Some("<".to_string())]),
            direction: Some(vec![1]),
            solution_data_types: vec![
                        SolutionDataTypes::BitBinary(BitBinary::new()),
                        SolutionDataTypes::Integer(Integer::new(Some(10), Some(20))),
                        SolutionDataTypes::Real(Real::new(Some(10.0), Some(20.0))),
                        SolutionDataTypes::Integer(Integer::new(Some(-100), Some(20))),
                        SolutionDataTypes::Real(Real::new(Some(-10.0), Some(20.0))),
                    ],
            objective_function: parabloid_5_loc
        };  
  

        let mut solution_1 = Solution {problem: &problem, solution: vec![1.0, 2.0, -3.0, 4.0, 5.0], 
            objective_fitness_values: Vec::with_capacity(*problem.number_of_objectives()), 
            constraint_values: Vec::with_capacity(*problem.number_of_objectives()), 
            constraint_violation: 0, feasible: false, evaluated: false}; 

        let mut solution_2 = Solution {problem: &problem, solution: vec![12.0, 10.0, -3.0, 4.0, 5.0], 
        objective_fitness_values: Vec::with_capacity(*problem.number_of_objectives()), 
        constraint_values: Vec::with_capacity(*problem.number_of_objectives()), 
        constraint_violation: 0, feasible: false, evaluated: false}; 

        let mut solution_3 = Solution {problem: &problem, solution: vec![-22.0, 12.0, -3.0, 1.0, 5.0], 
        objective_fitness_values: Vec::with_capacity(*problem.number_of_objectives()), 
        constraint_values: Vec::with_capacity(*problem.number_of_objectives()), 
        constraint_violation: 0, feasible: false, evaluated: false};

        let mut solution_4 = Solution {problem: &problem, solution: vec![-22.0, 12.0, -3.0, 1.0, 5.0], 
        objective_fitness_values: Vec::with_capacity(*problem.number_of_objectives()), 
        constraint_values: Vec::with_capacity(*problem.number_of_objectives()), 
        constraint_violation: 0, feasible: false, evaluated: false};

        let mut solution_5 = Solution {problem: &problem, solution: vec![1.0, 2.0, 3.0, 4.0, 5.0], 
        objective_fitness_values: Vec::with_capacity(*problem.number_of_objectives()), 
        constraint_values: Vec::with_capacity(*problem.number_of_objectives()), 
        constraint_violation: 0, feasible: false, evaluated: false};

        solution_1.evaluate();
        solution_2.evaluate();
        solution_3.evaluate();
        solution_4.evaluate();
        solution_5.evaluate();    

        let population = vec![&solution_1, &solution_2, &solution_3, &solution_4, & solution_5];
        // print memory size of solution_1
        println!("Memory size of solution_1: {}", mem::size_of_val(&solution_1));
        let tournament_selector = TournamentSelector::default();
        let winner = tournament_selector.select_one(&population);
        println!("Winner: {:?}", winner);   


    }

    #[test]
    fn test_tournament_selector_multi_objective() {
            
        let problem: Problem = Problem {
            solution_length: 5,
            number_of_objectives: 1,
            objective_constraint: Some(vec![Some(10.0)]),
            objective_constraint_operands: Some(vec![Some("<".to_string())]),
            direction: Some(vec![1]),
            solution_data_types: vec![
                        SolutionDataTypes::BitBinary(BitBinary::new()),
                        SolutionDataTypes::Integer(Integer::new(Some(10), Some(20))),
                        SolutionDataTypes::Real(Real::new(Some(10.0), Some(20.0))),
                        SolutionDataTypes::Integer(Integer::new(Some(-100), Some(20))),
                        SolutionDataTypes::Real(Real::new(Some(-10.0), Some(20.0))),
                    ],
            objective_function: parabloid_hyper_5
        };      

        let mut solution_1 = Solution {problem: &problem, solution: vec![1.0, 2.0, 3.0, 4.0, 5.0], 
                                                objective_fitness_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_violation: 0, feasible: false, evaluated: false}; 
          
        let mut solution_2 = Solution {problem: &problem, solution: vec![12.0, 10.0, -3.0, 4.0, 5.0], 
            objective_fitness_values: Vec::with_capacity(*problem.number_of_objectives()), 
            constraint_values: Vec::with_capacity(*problem.number_of_objectives()), 
            constraint_violation: 0, feasible: false, evaluated: false}; 
        
        let mut solution_3 = Solution {problem: &problem, solution: vec![-22.0, 12.0, -3.0, 1.0, 5.0], 
            objective_fitness_values: Vec::with_capacity(*problem.number_of_objectives()), 
            constraint_values: Vec::with_capacity(*problem.number_of_objectives()), 
            constraint_violation: 0, feasible: false, evaluated: false};

        let mut solution_4 = Solution {problem: &problem, solution: vec![-22.0, 12.0, -3.0, 1.0, 5.0], 
            objective_fitness_values: Vec::with_capacity(*problem.number_of_objectives()), 
            constraint_values: Vec::with_capacity(*problem.number_of_objectives()), 
            constraint_violation: 0, feasible: false, evaluated: false};

        let mut solution_5 = Solution {problem: &problem, solution: vec![1.0, 2.0, 3.0, 4.0, 5.0], 
            objective_fitness_values: Vec::with_capacity(*problem.number_of_objectives()), 
            constraint_values: Vec::with_capacity(*problem.number_of_objectives()), 
            constraint_violation: 0, feasible: false, evaluated: false};

        solution_1.evaluate();
        solution_2.evaluate();
        solution_3.evaluate();
        solution_4.evaluate();
        solution_5.evaluate();


        let population = vec![&solution_1, &solution_2, &solution_3, &solution_4, & solution_5];
        // print memory size of solution_1
        println!("Memory size of solution_1: {}", mem::size_of_val(&solution_1));
        let tournament_selector = TournamentSelector::default();
        let winner = tournament_selector.select_one(&population);
        println!("Winner: {:?}", winner);   


    }

    #[test]
    fn test_select_function() {
        let problem: Problem = Problem {
            solution_length: 3,
            number_of_objectives: 1,
            objective_constraint: Some(vec![Some(10.0)]),
            objective_constraint_operands: Some(vec![Some("<".to_string())]),
            direction: Some(vec![1]),
            solution_data_types: vec![
                        SolutionDataTypes::BitBinary(BitBinary::new()),
                        SolutionDataTypes::Integer(Integer::new(Some(10), Some(20))),
                        SolutionDataTypes::Real(Real::new(Some(10.0), Some(20.0))),
                        SolutionDataTypes::Integer(Integer::new(Some(-100), Some(20))),
                        SolutionDataTypes::Real(Real::new(Some(-10.0), Some(20.0))),
                    ],
            objective_function: parabloid_5_loc
        };      

        let mut solution_1 = Solution {problem: &problem, solution: vec![1.0, 2.0, 3.0, 4.0, 5.0], 
                                                objective_fitness_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_violation: 0, feasible: false, evaluated: false}; 

        let mut solution_2 = Solution {problem: &problem, solution: vec![12.0, 10.0, -3.0, 4.0, 5.0], 
                                                objective_fitness_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_violation: 0, feasible: false, evaluated: false}; 

        let mut solution_3 = Solution {problem: &problem, solution: vec![-22.0, 12.0, -3.0, 1.0, 5.0], 
                                                objective_fitness_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_violation: 0, feasible: false, evaluated: false};

        let mut solution_4 = Solution {problem: &problem, solution: vec![-22.0, 12.0, -3.0, 1.0, 5.0], 
                                                objective_fitness_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_violation: 0, feasible: false, evaluated: false};

        let mut solution_5 = Solution {problem: &problem, solution: vec![1.0, 2.0, 3.0, 4.0, 5.0], 
                                                objective_fitness_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_values: Vec::with_capacity(*problem.number_of_objectives()), 
                                                constraint_violation: 0, feasible: false, evaluated: false};
        solution_1.evaluate();
        solution_2.evaluate();
        solution_3.evaluate();
        solution_4.evaluate();
        solution_5.evaluate();

        let tournament_selector = TournamentSelector::default();
        let population: Vec<&Solution>  = vec![&solution_1, &solution_2, &solution_3, &solution_4, & solution_5];
        let winners: Vec<&Solution> = tournament_selector.select(10, &population);
        println!("Winners: {:?}", winners);
        println!("Winners: {:?}", winners.len());
        // assert that the length of the winners is equal to the number of winners
        assert_eq!(winners.len(), 10);

        
    }
}




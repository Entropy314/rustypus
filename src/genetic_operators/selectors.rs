use crate::dominance::ParetoDominance;
use crate::core::Solution; 
use crate::dominance::Dominance;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

pub trait Selector { 
    fn select<'a>(&self, population: &[&'a Solution<'a>], n: usize) -> Vec<&'a Solution<'a>>;
    fn select_one<'a>(&self, population: &[&'a Solution<'a>]) -> &'a Solution<'a>;
}


#[derive(Debug)]
pub struct TournamentSelector {
    tournament_size: usize,
    dominance: ParetoDominance,
    rng: StdRng, // Random number generator with optional seed
}

impl TournamentSelector {

    /// Default constructor with random seed
    pub fn new(tournament_size: usize, dominance: ParetoDominance, seed: Option<u64>) -> Self {
        let rng = match seed {
            Some(seed_value) => StdRng::seed_from_u64(seed_value), // Seeded RNG
            None => StdRng::from_entropy(),                      // RNG with entropy
        };

        TournamentSelector {
            tournament_size,
            dominance,
            rng,
        }
    }

    pub fn select_one<'a>(&mut self, population: &[&'a Solution<'a>]) -> &'a Solution<'a> {
        let mut winner = population[self.rng.gen_range(0..population.len())];

        for _ in 0..self.tournament_size {
            let challenger = population[self.rng.gen_range(0..population.len())];
            let flag = self.dominance.compare_solutions(challenger, winner);
            if flag > 0 {
                winner = challenger;
            }
        }

        winner
    }

    pub fn select<'a>(&'a mut self, n: usize, population: &[&'a Solution<'a>]) -> Vec<&'a Solution<'a>> {
        let mut results = Vec::with_capacity(n);
        for _ in 0..n {
            let winner = {
                self.select_one(population) // Borrow is confined to this block
            };
            results.push(winner);
        }
        results
    }
}


impl Default for TournamentSelector {
    fn default() -> Self {
        TournamentSelector::new(2, ParetoDominance, Some(1234))
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

    fn setup_problem(objective_function: fn(&Vec<f64>) -> Vec<f64>, direction: Vec<i8>) -> Problem {
        Problem {
            solution_length: 5,
            number_of_objectives: 1,
            objective_constraint: Some(vec![Some(10.0)]),
            objective_constraint_operands: Some(vec![Some("<".to_string())]),
            direction: Some(direction),
            solution_data_types: vec![
                SolutionDataTypes::BitBinary(BitBinary::new()),
                SolutionDataTypes::Integer(Integer::new(Some(10), Some(20))),
                SolutionDataTypes::Real(Real::new(Some(10.0), Some(20.0))),
                SolutionDataTypes::Integer(Integer::new(Some(-100), Some(20))),
                SolutionDataTypes::Real(Real::new(Some(-10.0), Some(20.0))),
            ],
            objective_function,
        }
    }

    fn setup_solutions(problem: &Problem) -> Vec<Solution> {
        vec![
            Solution {
                problem,
                solution: vec![1.0, 2.0, -3.0, 4.0, 5.0],
                objective_fitness_values: Vec::with_capacity(*problem.number_of_objectives()),
                constraint_values: Vec::with_capacity(*problem.number_of_objectives()),
                constraint_violation: 0,
                feasible: false,
                evaluated: false,
            },
            Solution {
                problem,
                solution: vec![12.0, 10.0, -3.0, 4.0, 5.0],
                objective_fitness_values: Vec::with_capacity(*problem.number_of_objectives()),
                constraint_values: Vec::with_capacity(*problem.number_of_objectives()),
                constraint_violation: 0,
                feasible: false,
                evaluated: false,
            },
            Solution {
                problem,
                solution: vec![-22.0, 12.0, -3.0, 1.0, 5.0],
                objective_fitness_values: Vec::with_capacity(*problem.number_of_objectives()),
                constraint_values: Vec::with_capacity(*problem.number_of_objectives()),
                constraint_violation: 0,
                feasible: false,
                evaluated: false,
            },
            Solution {
                problem,
                solution: vec![1.0, 2.0, 3.0, 4.0, 5.0],
                objective_fitness_values: Vec::with_capacity(*problem.number_of_objectives()),
                constraint_values: Vec::with_capacity(*problem.number_of_objectives()),
                constraint_violation: 0,
                feasible: false,
                evaluated: false,
            },
        ]
    }

    fn evaluate_solutions(solutions: &mut Vec<Solution>) {
        for solution in solutions.iter_mut() {
            solution.evaluate();
        }
    }

    #[test]
    fn test_tournament_selector_single_objective_maximize() {
        let problem = setup_problem(parabloid_5_loc, vec![1]);
        let mut solutions = setup_solutions(&problem);
        evaluate_solutions(&mut solutions);

        let population: Vec<&Solution> = solutions.iter().collect();
        let mut tournament_selector = TournamentSelector::default();
        let winner = tournament_selector.select_one(&population);

        println!("Memory size of a solution: {} bytes", mem::size_of_val(&solutions[0]));
        println!("Winner: {:?}", winner);
    }

    #[test]
    fn test_tournament_selector_single_objective_miminmize() {
        let problem = setup_problem(parabloid_5_loc, vec![-1]);
        let mut solutions = setup_solutions(&problem);
        evaluate_solutions(&mut solutions);

        let population: Vec<&Solution> = solutions.iter().collect();
        let mut tournament_selector = TournamentSelector::default();
        let winner = tournament_selector.select_one(&population);

        println!("Memory size of a solution: {} bytes", mem::size_of_val(&solutions[0]));
        println!("Winner: {:?}", winner);
    }

    #[test]
    fn test_tournament_selector_multi_objective_maximize() {
        let problem = setup_problem(parabloid_hyper_5, vec![1, 1, 1, 1, 1]);
        let mut solutions = setup_solutions(&problem);
        evaluate_solutions(&mut solutions);

        let population: Vec<&Solution> = solutions.iter().collect();
        let mut tournament_selector = TournamentSelector::default();
        let winner = tournament_selector.select_one(&population);

        println!("Memory size of a solution: {} bytes", mem::size_of_val(&solutions[0]));
        println!("Winner: {:?}", winner);
    }

    #[test]
    fn test_selection_function() {
        let problem = setup_problem(parabloid_hyper_5, vec![-1, -1, -1, -1, -1]);
        let mut solutions = setup_solutions(&problem);
        evaluate_solutions(&mut solutions);

        let population: Vec<&Solution> = solutions.iter().collect();
        let mut tournament_selector = TournamentSelector::default();
        let winners = tournament_selector.select(2, &population);

        println!("Memory size of a solution: {} bytes", mem::size_of_val(&solutions[0]));
        println!("Winners: {:?}", winners);
    }
}
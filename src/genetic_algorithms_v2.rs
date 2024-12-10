use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::core::{Problem, Solution};
use crate::genetic_operators::mutation::MutationManager;
use crate::genetic_operators::crossover::CrossoverManager;
use crate::genetic_operators::selectors::TournamentSelector;
use crate::dominance::DominanceEnum;

pub trait GeneticAlgorithm<'a> {
    fn initialize(&mut self);
    // fn iterate(&mut self);
    fn evaluate_all(&mut self);
    fn add_solution(&mut self, solution: Solution<'a>);
}

pub struct BaseGeneticAlgorithm<'a> {
    pub problem: &'a Problem,
    pub parent_population_size: usize,
    pub offspring_population_size: usize,
    pub parent_population: Vec<Solution<'a>>,
    pub offspring_population: Vec<Solution<'a>>,
    pub nfe: AtomicUsize,
    pub selector: TournamentSelector,
    pub dominance: DominanceEnum,
    pub mutation_manager: MutationManager<'a>,
    pub crossover_manager: CrossoverManager<'a>,
    pub archive: Vec<Solution<'a>>,
}

impl<'a> BaseGeneticAlgorithm<'a> {
    pub fn new(
        problem: &'a Problem,
        parent_population_size: usize,
        offspring_population_size: usize,
    ) -> Self {
        Self {
            problem,
            parent_population_size,
            offspring_population_size,
            parent_population: Vec::with_capacity(parent_population_size),
            offspring_population: Vec::with_capacity(offspring_population_size),
            nfe: AtomicUsize::new(0),
            selector: TournamentSelector::default(),
            dominance: DominanceEnum::ParetoDominance,
            mutation_manager: MutationManager::new(),
            crossover_manager: CrossoverManager::new(),
            archive: Vec::with_capacity(parent_population_size),
        }
    }

    fn archive_solution(&mut self, solution: Solution<'a>) {
        if solution.feasible && solution.evaluated {
            self.archive.push(solution);
        }
    }
}

impl<'a> GeneticAlgorithm<'a> for BaseGeneticAlgorithm<'a> {
    fn initialize(&mut self) {
        self.parent_population = (0..self.parent_population_size)
            .into_par_iter()
            .map(|_| {
                let mut solution = Solution::new(self.problem);
                solution.solution = self.problem.generate_solution();
                solution
            })
            .collect();
        println!("Initialized parent population {:?}", self.parent_population);

    }

  
    fn evaluate_all(&mut self) {
        let new_evaluations: usize = self.parent_population
            .par_iter_mut()
            .filter(|solution| !solution.evaluated)
            .map(|solution| {
                solution.evaluate();
                1
            })
            .sum();
        self.nfe.fetch_add(new_evaluations, Ordering::SeqCst);
    }

    fn add_solution(&mut self, solution: Solution<'a>) {
        self.parent_population.push(solution);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Problem, Solution};
    use crate::genetic_operators::mutation::MutationManager;
    use crate::genetic_operators::crossover::CrossoverManager;
    use crate::genetic_operators::selectors::TournamentSelector;
    use crate::dominance::DominanceEnum;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use crate::gatypes::SolutionDataTypes;
    use crate::gatypes::{BitBinary, Integer, Real};
    fn setup_problem() -> Problem {
        Problem {
            solution_length: 5,
            number_of_objectives: 1,
            objective_constraint: Some(vec![Some(10.0)]),
            objective_constraint_operands: Some(vec![Some("<".to_string())]),
            direction: Some(vec![1]),
            solution_data_types: vec![
                SolutionDataTypes::BitBinary(BitBinary::new()),
                SolutionDataTypes::Integer(Integer::new(Some(10), Some(2000))),
                SolutionDataTypes::Real(Real::new(Some(10.0), Some(1000.0))),
                SolutionDataTypes::Real(Real::new(Some(10.0), Some(1000.0))),
                SolutionDataTypes::Real(Real::new(Some(10.0), Some(1000.0))),
            ],
            objective_function: |x| vec![x.iter().sum()],
        }
    }
    
    #[test]
    fn test_base_genetic_algorithm() {
        let problem = setup_problem();
        let num: usize = 3; 
        let mut ga = BaseGeneticAlgorithm::new(&problem, num, num);
        ga.initialize();
        ga.evaluate_all();
        // print ga.nfe.load(Ordering::SeqCst)) with commas
        println!("\n"); 
        println!("FINALS {:?}", ga.parent_population);

        // print then \n 
        // for i in 0..ga.parent_population_size {
            // println!("{:?}", i);
            // println!("{:?}", ga.parent_population[i]);
            // println!("\n"); 
        // }
        // println!("LENGTH"); 
        // println!("{:?}", ga.parent_population.len());
        // assert_eq!(ga.parent_population.len(),  num);
    }
}
use crate::core::{Problem, Solution};
use crate::genetic_operators::{Variation, GeneticVariator}; 
use crate::selectors::{Selector, TournamentSelector};
use crate::benchmark_objective_functions::parabloid_5_loc;
use crate::gatypes::{SolutionDataTypes, Real, Integer, BitBinary};
use crate::dominance::DominanceEnum;


pub trait AbstractGeneticAlgorithm {
    fn step(&mut self);
    fn initialize(&mut self);
    fn iterate(&mut self);
    fn evaluate_all(&mut self, population: &mut Vec<Solution>);
}

pub struct BaseGeneticAlgorithm<'a> {
    pub problem: &'a Problem,
    pub population_size: usize,
    pub population: Vec<Solution<'a>>,
    pub offspring_size: usize,
    pub offspring: Vec<Solution<'a>>,
    pub nfe: usize,
    pub selector: TournamentSelector, 
    pub dominance: DominanceEnum,
    pub variator: GeneticVariator,
}

impl<'a> BaseGeneticAlgorithm<'a> {
    pub fn new(problem: &'a Problem, population_size: Option<usize>, offspring_size:Option<usize>) -> Self {
        let population_size: usize = population_size.unwrap_or(10);
        let offspring_size: usize = offspring_size.unwrap_or(10);
        Self {
            problem,
            population_size,     
            population: Vec::with_capacity(population_size),
            offspring_size: offspring_size,
            nfe: 0,
            dominance: DominanceEnum::ParetoDominance, 
            selector: TournamentSelector::default(),
            variator: GeneticVariator,
        }
    }
}

impl<'a> AbstractGeneticAlgorithm for BaseGeneticAlgorithm<'a> {
    fn step(&mut self) {
        if self.nfe == 0 {
            self.initialize();
        } else {
            self.iterate();
        }
    }

    fn initialize(&mut self) {
        for _ in 0..self.population_size {
            let mut solution = Solution::new(self.problem);
            solution.solution = self.problem.generate_solution();
            self.population.push(solution);
        }
    }

    fn iterate(&mut self) {


    }

    fn evaluate_all(&mut self, population: &mut Vec<Solution>) {
        for individual in population.iter_mut() {
            if !individual.evaluated {
                // Evaluate the individual here
                // Example: individual.objective_fitness_values = self.problem.evaluate(&individual.solution);
                individual.evaluated = true;
                self.nfe += 1; // Increment the number of function evaluations
            }
        }
    }
}



// UnitTests

#[cfg(test)]
// Test BaseGeneticAlgorithm

mod tests {
    use super::*;
    use crate::benchmark_objective_functions::parabloid_5_loc;
    use crate::core::{Problem, Solution};

    #[test]
    fn test_base_genetic_algorithm() {
        let solution_data_types = vec![
            SolutionDataTypes::BitBinary(BitBinary::new()),
            SolutionDataTypes::Integer(Integer::new(Some(10), Some(20))),
            SolutionDataTypes::Real(Real::new(Some(10.0), Some(20.0))),
            SolutionDataTypes::Integer(Integer::new(Some(10), Some(20))),
            SolutionDataTypes::Real(Real::new(Some(10.0), Some(20.0))),
        ];
    
        let problem = Problem::new(
            5,
            1,
            None,
            None,
            None,
            solution_data_types,
            parabloid_5_loc,
        );
    
        let solution = Solution::new(&problem);
        let mut ga = BaseGeneticAlgorithm::new(&problem, 100);
        ga.step();
        assert_eq!(ga.population.len(), 100);
    }
}



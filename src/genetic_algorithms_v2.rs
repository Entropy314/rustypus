use crate::core::{Problem, Solution};
use crate::genetic_operators::{
    mutation::{MutationManager, BitFlipMutation, PolynomialMutation},
    crossover::{CrossoverManager, SimulatedBinaryCrossover, UniformCrossover},
    selectors::{Selector, TournamentSelector},
};
use crate::benchmark_objective_functions::parabloid_5_loc;
use crate::gatypes::{SolutionDataTypes, Real, Integer, BitBinary};
use crate::dominance::DominanceEnum;

pub trait AbstractGeneticAlgorithm<'a> {
    /// Perform a single step of the genetic algorithm.
    fn step(&mut self);
    /// Initialize the algorithm by creating the parent population.
    fn initialize(&mut self);
    /// Perform one iteration of the algorithm (selection, variation, evaluation).
    fn iterate(&mut self);
    /// Evaluate all individuals in the parent population.
    fn evaluate_all(&mut self);
    /// Add a solution to the parent population.
    fn add_solution(&mut self, solution: Solution<'a>);
}

pub struct BaseGeneticAlgorithm<'a> {
    pub problem: &'a Problem,
    pub parent_population_size: usize,
    pub offspring_population_size: usize,
    pub parent_population: Vec<Solution<'a>>,
    pub offspring_population: Vec<Solution<'a>>,
    pub nfe: usize,
    pub selector: TournamentSelector,
    pub dominance: DominanceEnum,
    pub archive: Vec<Solution<'a>>,
    pub mutation_manager: MutationManager<'a>,
    pub crossover_manager: CrossoverManager<'a>,
}

impl<'a> BaseGeneticAlgorithm<'a> {
    pub fn new(
        problem: &'a Problem,
        parent_population_size: Option<usize>,
        offspring_population_size: Option<usize>,
        mutation_manager: Option<MutationManager<'a>>,
        crossover_manager: Option<CrossoverManager<'a>>,
    ) -> Self {
        let parent_population_size = parent_population_size.unwrap_or(10);
        let offspring_population_size = offspring_population_size.unwrap_or(10);

        let default_mutation_manager = MutationManager::new();
        let default_crossover_manager = CrossoverManager::new();

        Self {
            problem,
            parent_population_size,
            offspring_population_size,
            parent_population: Vec::new(),
            offspring_population: Vec::new(),
            nfe: 0,
            selector: TournamentSelector::default(),
            dominance: DominanceEnum::ParetoDominance,
            mutation_manager: mutation_manager.unwrap_or(default_mutation_manager),
            crossover_manager: crossover_manager.unwrap_or(default_crossover_manager),
            archive: Vec::new(),
        }
    }

    /// Archive a solution if it is feasible and evaluated
    fn archive_solution(&mut self, solution: Solution<'a>) {
        if solution.feasible && solution.evaluated {
            self.archive.push(solution);
        }
    }
}

impl<'a> AbstractGeneticAlgorithm<'a> for BaseGeneticAlgorithm<'a> {
    fn step(&mut self) {
        if self.nfe == 0 {
            self.initialize();
        } else {
            self.iterate();
        }
    }

    fn initialize(&mut self) {
        self.parent_population = (0..self.parent_population_size)
            .map(|_| {
                let mut solution = Solution::new(self.problem);
                solution.solution = self.problem.generate_solution();
                solution.evaluate();
                self.nfe += 1;
                solution
            })
            .collect();
    }

    fn iterate(&mut self) {
        while self.offspring_population.len() < self.offspring_population_size {
            // Select parents
            let parents: Vec<&Solution> = self
                .selector
                .select(2, &self.parent_population.iter().collect::<Vec<_>>());

            // Apply crossover
            let (mut child1, mut child2) = self
                .crossover_manager
                .perform_crossover(parents[0], parents[1]);

            // Apply mutation
            {
                let mut child1_mut = self.mutation_manager.mutate(&child1);
                let mut child2_mut = self.mutation_manager.mutate(&child2);
            }
            
            
            

            // Evaluate children
            for child in [&mut child1, &mut child2] {
                if !child.evaluated {
                    child.evaluate();
                    self.nfe += 1;
                }
            }

            // Archive and add offspring
            self.archive_solution(child1.clone());
            self.archive_solution(child2.clone());
            self.offspring_population.push(child1);
            self.offspring_population.push(child2);
        }

        // Replace parent population
        self.parent_population = std::mem::take(&mut self.offspring_population);
    }

    fn add_solution(&mut self, solution: Solution<'a>) {
        self.parent_population.push(solution);
    }

    fn evaluate_all(&mut self) {
        self.parent_population.iter_mut().for_each(|solution| {
            if !solution.evaluated {
                solution.evaluate();
                self.nfe += 1;
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::benchmark_objective_functions::parabloid_5_loc;
    use crate::core::{Problem, Solution};

    #[test]
    fn test_initialize_and_iterate() {
        let solution_data_types = vec![
            SolutionDataTypes::BitBinary(BitBinary::new()),
            SolutionDataTypes::Integer(Integer::new(Some(10), Some(20))),
            SolutionDataTypes::Real(Real::new(Some(10.0), Some(20.0))),
        ];

        let problem = Problem::new(
            3,
            1,
            None,
            None,
            None,
            solution_data_types,
            parabloid_5_loc,
        );

        let mutation_manager = MutationManager::new();
        let crossover_manager = CrossoverManager::new();

        let mut ga = BaseGeneticAlgorithm::new(
            &problem,
            Some(10),
            Some(10),
            Some(mutation_manager),
            Some(crossover_manager),
        );

        ga.initialize();

        // Verify initialization
        assert_eq!(ga.parent_population.len(), 10);
        assert_eq!(ga.nfe, 10);

        // Iterate the algorithm
        ga.iterate();

        // Verify offspring population and evaluations
        assert_eq!(ga.parent_population.len(), 10);
        assert!(ga.parent_population.iter().all(|s| s.evaluated));
    }
}
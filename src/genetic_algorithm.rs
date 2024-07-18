use crate::dominance::ParetoDominance; 
use crate::selectors::TournamentSelector;
use crate::core::{Problem, Solution};

pub trait GeneticAlgorithm { 
    fn initialize(&mut self); 
    fn evaluate_all_populations(&mut self); 
    fn select_parents(&mut self); 
    fn crossover(&mut self); 
    fn mutate(&mut self); 
    fn select_survivors(&mut self); 
    fn evolve(&mut self); 
    fn run(&mut self); 

}

#[derive(Debug)]
pub struct CoreGeneticAlgorithm<'a> {
    generation: usize,
    population_size: usize,
    offspring_size: usize,
    selector: TournamentSelector,
    problem: &'a Problem,
    population: Vec<Solution<'a>>,
    offspring: Vec<Solution<'a>>,
}

impl<'a> CoreGeneticAlgorithm<'a> {
    pub fn new(problem: &'a Problem) -> Self {
        Self {
            generation: 0,
            population_size: 100,
            offspring_size: 100,
            selector: TournamentSelector::default(),
            problem: &problem,
            population: Vec::with_capacity(100),
            offspring: Vec::with_capacity(100),
        }
    }

    pub fn initialize(&mut self) {
        for _ in 0..self.population_size {
            let solution = Solution::new(self.problem);
            self.population.push(solution);
        }
    }

    pub fn evaluate_all_populations(&mut self) {
        for solution in self.population.iter_mut() {
            solution.evaluate();
        }
    }
}


// Struct NSGAII population_size 100
struct NSGAII<'a> {
    n_generations: usize,
    generation: usize,
    population_size: usize,
    offspring_size: usize,
    selector: TournamentSelector,
    problem: &'a Problem,
    population: Vec<Solution<'a>>,
    offspring: Vec<Solution<'a>>,
    variator: Vec
}

impl<'a> NSGAII<'a> {
    pub fn new(problem: &'a Problem) -> Self {
        Self {
            n_generations: 100,
            generation: 0,
            population_size: 100,
            offspring_size: 100,
            selector: TournamentSelector::default(),
            problem: &problem,
            population: Vec::with_capacity(100),
            offspring: Vec::with_capacity(100),
        }
    }

    pub fn initialize(&mut self) {
        for _ in 0..self.population_size {
            let solution = Solution::new(self.problem);
            self.population.push(solution);
        
        }
    }

    pub fn evaluate_all_populations(&mut self) {
        for solution in self.population.iter_mut() {
            solution.evaluate();
        }
    }

    pub fn iterate(&mut self) { 
        while self.offspring.len() < self.population_size {
            self.generation += 1;
        }
    }
    
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::benchmark_objective_functions::parabloid_5_loc;
    use crate::core::{Solution, Problem};
    use crate::gatypes::{SolutionType, Binary, Integer, Real};

    #[test]
    fn test_genetic_algorithm() {
        let problem = Problem {
            solution_length: 3,
            number_of_objectives: 1,
            objective_constraint: Some(vec![Some(10.0)]),
            objective_constraint_operands: Some(vec![Some("<".to_string())]),
            direction: Some(vec![1]),
            solution_data_type: vec![SolutionType::Binary, SolutionType::Integer, SolutionType::Real],
            objective_function: parabloid_5_loc
        };
        
        let mut ga = CoreGeneticAlgorithm::new(&problem);
        ga.initialize();
        println!("HELLLOOO"); 
        println!("{:?}", ga.population);
        ga.evaluate_all_populations();
        println!("BYEEEEEE"); 
        println!("{:?}", ga.population);

        assert!(ga.population.len() == 100);
    }
}

mod individual;
mod population;

use population::{Population, PopulationParams};


pub type FitnessFunction = fn(f32) -> f32;

pub struct OptimizationTask {
    pub min_bound: f32,
    pub max_bound: f32,
    pub mutation_probability: f32,
    pub population_size: usize,
    pub fitness_function: FitnessFunction,
    pub generations: u32,
}

impl OptimizationTask {
    fn to_population_params(&self) -> PopulationParams {
        PopulationParams{
            min_bound: self.min_bound,
            max_bound: self.max_bound,
            mutation_probability: self.mutation_probability,
            population_size: self.population_size,
            fitness_function: self.fitness_function
        }
    }
}

pub fn optimize(task: OptimizationTask) -> f32 {
    let mut population = Population::new(task.to_population_params());

    for _ in 0..task.generations {
        population = population.next_generation();
    }

    population.fittest().getX()
}

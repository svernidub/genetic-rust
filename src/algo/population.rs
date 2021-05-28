use super::{individual::Individual, FitnessFunction};
use std::fmt::{Display, Formatter};
use std::fmt;
use rand::thread_rng;
use rand::prelude::SliceRandom;

pub(super) struct Population {
    population: Vec<Individual>,
    mutation_probability: f32,
    fitness_function: FitnessFunction,
    min_bound: f32,
    max_bound: f32,
    size: usize,
    left: Individual,
    right: Individual
}

pub(super) struct PopulationParams {
    pub min_bound: f32,
    pub max_bound: f32,
    pub mutation_probability: f32,
    pub population_size: usize,
    pub fitness_function: FitnessFunction
}

impl Population {
    pub(super) fn new(params: PopulationParams) -> Self {
        let mut population = vec!();

        for _ in 0..params.population_size {
            population.push(Individual::random(params.min_bound, params.max_bound, params.fitness_function));
        }

        Self{
            population,
            min_bound: params.min_bound,
            max_bound: params.max_bound,
            mutation_probability: params.mutation_probability,
            fitness_function: params.fitness_function,
            size: params.population_size,
            left: Individual::new(params.min_bound, params.fitness_function),
            right: Individual::new(params.max_bound, params.fitness_function)
        }
    }

    pub(super) fn next_generation(&self) -> Population {
        let mut descendants = self.population.clone();
        let mut t = thread_rng();
        descendants.shuffle(&mut t);
        self.produce_descendants(&mut descendants);
        descendants.sort();
        descendants.reverse();

        Self{
            population: descendants[0..self.size].to_owned(),
            mutation_probability: self.mutation_probability,
            fitness_function: self.fitness_function,
            min_bound: self.min_bound,
            max_bound: self.max_bound,
            size: self.size,
            left: self.left,
            right: self.right
        }
    }

    fn produce_descendants(&self, descendants: &mut Vec<Individual>) {
        for chunk in self.population.chunks_exact(2) {
            match *chunk {
                [ancestor1, ancestor2] => {
                    let [descendant1, descendant2] = ancestor1.mate(ancestor2, self.mutation_probability);
                    self.append_child(descendants, descendant1);
                    self.append_child(descendants, descendant2);
                }
                [_] => (),
                _ => unreachable!("Chunk size will 2")
            }
        }
    }

    pub(super) fn fittest(&self) -> Individual {
        let mut pop = self.population.clone();
        pop.sort();
        pop.reverse();
        pop.first().unwrap().clone()
    }

    fn append_child(&self, descendants: &mut Vec<Individual>, child: Individual) {
        if child.lefter_then(&self.left) { return; }
        if child.righter_then(&self.right) { return; }
        if child.invalid() { return; }

        descendants.push(child);
    }
}

impl Display for Population {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[");

        self.population.iter().for_each(|x| {
            write!(f, "{}", x);
        });

        write!(f, "]")
    }
}

#[cfg(test)]
mod test {
    use super::Population;

    fn sin(x: f32) -> f32 { x.sin() }


    #[test]
    fn test_create_population() {
        let p = Population::new(0.0, 3.2, 0.1, 100, sin);
        assert_eq!(p.population.len(), 100);

        for individual in p.population.iter() {
            assert!(individual.lefter_then(&p.right));
            assert!(individual.righter_then(&p.left));
        }
    }

    #[test]
    fn test_next_generation() {
        let p1 = Population::new(0.0, 3.2, 0.1, 100, sin);
        let p2 = p1.next_generation();

        let mut diff = 0u32;
        p1.population.iter().zip(p2.population.iter()).for_each(|(i1, i2)| {
            if i1 != i2 {
                diff += 1;
            }
        });

        assert_ne!(diff, 0);
    }
}

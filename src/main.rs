use std::f32::consts::PI;

mod algo;

fn sin(x: f32) -> f32 {
    x.sin()
}

fn main() {
    let result = algo::optimize(algo::OptimizationTask{
        min_bound: 0.0,
        max_bound: PI,
        mutation_probability: 0.5,
        population_size: 30000  ,
        fitness_function: sin,
        generations: 100
    });

    println!("Result: {}", result)
}

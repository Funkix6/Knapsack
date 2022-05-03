mod initial_population;
mod fitness_function;
mod selection;
mod crossover;
mod mutation;
mod context;

use crate::initial_population::Population;
use crate::fitness_function::evaluate_fitness;
use crate::context::Backpack;

fn main() {

    let backpack = Backpack::new(6, 10);
  
    initial_population::run();

    let mut population = Population::new(10, 6);

    population.print_population();

    fitness_function::run();
  
    population = fitness_function::evaluate_fitness(population, &backpack);

    population.chromosomes.sort_by(|a, b| b.fitness.cmp(&a.fitness));
  
    population.print_population();
  
    selection::run();
    crossover::run();
    mutation::run();
}



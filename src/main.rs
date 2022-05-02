mod initial_population;
mod fitness_function;
mod selection;
mod crossover;
mod mutation;
mod context;

use crate::initial_population::Population;
use crate::context::Backpack;

fn main() {

    let backpack = Backpack::new(6, 20);

    // backpack.print_content();

    initial_population::run();

    let population = Population::new(10, 6);

    population.print_population();

    fitness_function::run();
    selection::run();
    crossover::run();
    mutation::run();
}



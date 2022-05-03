use crate::initial_population::Population;
use crate::initial_population::Chromosome;

use crate::context::Backpack;

pub fn run() {
    println!("Evaluating fitness...");
}

//Set up rules for fitness.

pub fn evaluate_fitness(population: Population) {
    
}

pub fn check_weight(chromosome: &Chromosome, backpack: &Backpack) {
    println!("{}", chromosome.gene[0]);
    backpack.print_content();
}


//Test the fitness of the populations


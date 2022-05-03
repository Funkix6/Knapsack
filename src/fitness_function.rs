use crate::initial_population::Population;
use crate::initial_population::Chromosome;
use crate::context::Backpack;

pub fn run() {
    println!("Evaluating fitness...");
}

//Set up rules for fitness.

pub fn evaluate_fitness(mut population: Population, backpack: &Backpack) -> Population {
  
  for i in 0..population.chromosomes.len() {    
      let weight = check_weight(&population.chromosomes[i], &backpack);
      let fitness: i16 = backpack.max_weight - weight;
      println!("fitness : {}", fitness);
      population.chromosomes[i].fitness = fitness;
    }

  population
}

//Test the fitness of the populations
pub fn check_weight(chromosome: &Chromosome, backpack: &Backpack) -> i16 {
  let mut total_weight: i16 = 0;
  
  for i in 0..chromosome.gene.len() {
    if chromosome.gene[i] == true {
      total_weight += backpack.items[i].weight;
      println!(
        "In the backpack there is : {} it weights {} kg.", 
        backpack.items[i].name, backpack.items[i].weight);
    }
  }
  println!("Total weight : {}", total_weight);
  total_weight
}

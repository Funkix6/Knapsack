use std::vec;

use rand::Rng;

pub fn run() {
    println!("Initializing population...");
}

pub struct Chromosome {
    pub gene: Vec<bool>,
    pub fitness: i32
}

pub struct Population {
    pub chromosomes: Vec<Chromosome>
}

impl Chromosome {
    pub fn new(size: i32) -> Chromosome {
        
        let mut gene = vec![];

        let fitness = 0;
      
        for _item in 0..size {
            let mut rng = rand::thread_rng();
            gene.push(rng.gen_bool(0.5));
        }
        
        Chromosome { 
            gene,
            fitness
        }
    }

    pub fn print_genes(&self) {
        print!("| Genes : | ");
        for item in self.gene.iter() {
            if *item { print!("1") } else { print!("0") }
            print!(" | ");
        } 

        println!();
    }
}

impl Population {
    pub fn new(size: i32, size_genes: i32) -> Population {

        let mut chromosomes = vec![];

        for _index in 0..size {
            //Create random chromosome.
            let chromosome = Chromosome::new(size_genes);
            
            // Push it in the current population.
            chromosomes.push(chromosome);
        }

        Population { 
            chromosomes
        }
    }

    pub fn print_population(&self){
        let mut index = 0;
        for item in self.chromosomes.iter() {
            print!("{} :", index);
            print!(" | Fitness : {} \t| \t ", item.fitness);
            item.print_genes();
            index += 1;
        } 
    }
}
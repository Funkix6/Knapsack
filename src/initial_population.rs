use std::vec;

use rand::Rng;

pub fn run() {
    println!("Initializing population...");
}

pub struct Chromosome {
    pub gene: Vec<bool>
}

pub struct Population {
    chromosomes: Vec<Chromosome>,
    fitness: u32,
}

impl Chromosome {
    pub fn new(size: u32) -> Chromosome {
        
        let mut gene = vec![];

        for _item in 0..size {
            let mut rng = rand::thread_rng();
            gene.push(rng.gen_bool(0.5));
        }
        
        Chromosome { 
            gene
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
    pub fn new(size: u32, size_genes: u32) -> Population {

        let mut chromosomes = vec![];

        for _index in 0..size {
            //Create random chromosome.
            let chromosome = Chromosome::new(size_genes);
            
            // Push it in the current population.
            chromosomes.push(chromosome);
        }

        Population { 
            chromosomes,
            fitness: 0
        }
    }

    pub fn print_population(&self){
        let mut index = 0;
        for item in self.chromosomes.iter() {
            print!("{} :", index);
            print!(" | Fitness : {} | ", self.fitness);
            item.print_genes();
            index += 1;
        } 
    }

}
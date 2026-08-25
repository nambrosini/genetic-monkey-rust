use rand::{RngExt, rngs::ThreadRng, seq::IndexedRandom};

struct Monkey {
    genes: Vec<char>,
    fitness: f64,
}

impl Monkey {
    fn new(len: usize) -> Self {
        let mut rng = rand::rng();
        Self {
            genes: (0..len).map(|_| generate_random_char(&mut rng)).collect(),
            fitness: 0f64,
        }
    }

    fn calc_fitness(&mut self, target: &str) {
        let score = self
            .genes
            .iter()
            .enumerate()
            .filter(|(i, g)| target.chars().nth(*i).unwrap() == **g)
            .count() as f64;

        self.fitness = score / target.len() as f64
    }

    fn crossover(&self, other: &Monkey) -> Monkey {
        let mut rng = rand::rng();
        let mut child = Monkey::new(self.genes.len());
        let midpoint = rng.random_range(0..self.genes.len());
        for i in 0..self.genes.len() {
            if i > midpoint {
                child.genes[i] = self.genes[i]
            } else {
                child.genes[i] = other.genes[i]
            }
        }
        child
    }

    fn mutate(&mut self, mutation_rate: f64) {
        let mut rng = rand::rng();
        for g in self.genes.iter_mut() {
            if rng.random::<f64>() < mutation_rate {
                *g = generate_random_char(&mut rng)
            }
        }
    }
}

fn generate_random_char(rng: &mut ThreadRng) -> char {
    rng.random_range(32..=128) as u8 as char
}

pub struct Population {
    pop_size: usize,
    mutation_rate: f64,
    pop: Vec<Monkey>,
    mating_pool: Vec<usize>,
    average_fitness: f64,
    target: String,
}

impl Population {
    pub fn new(target: &str, pop_size: usize, mutation_rate: f64) -> Self {
        Self {
            pop_size,
            mutation_rate,
            pop: (0..pop_size).map(|_| Monkey::new(target.len())).collect(),
            mating_pool: vec![],
            average_fitness: 0f64,
            target: target.to_string(),
        }
    }

    fn calc_fitness(&mut self) {
        self.pop
            .iter_mut()
            .for_each(|m| m.calc_fitness(&self.target));
        let total_fitness = self.pop.iter().fold(0f64, |acc, m| acc + m.fitness);
        self.average_fitness = total_fitness / self.pop_size as f64;
    }

    fn mate(&mut self) {
        self.mating_pool.clear();
        for (index, monkey) in self.pop.iter().enumerate() {
            let n = (monkey.fitness * 100f64).ceil() as usize;
            for _ in 0..n {
                self.mating_pool.push(index);
            }
        }
    }

    fn reproduce(&mut self) {
        let mut rng = rand::rng();
        let mut pop = vec![];
        for _ in &self.pop {
            let a = &self.pop[*self.mating_pool.choose(&mut rng).unwrap()];
            let b = &self.pop[*self.mating_pool.choose(&mut rng).unwrap()];
            let mut child = a.crossover(b);
            child.mutate(self.mutation_rate);
            pop.push(child);
        }
        self.mating_pool.clear();
        self.pop = pop;
    }

    pub fn get_average_fitness(&self) -> f64 {
        self.average_fitness
    }

    pub fn best_monkey(&self) -> String {
        self.pop
            .iter()
            .max_by(|a, b| a.fitness.total_cmp(&b.fitness))
            .unwrap()
            .genes
            .iter()
            .collect()
    }

    pub fn simulate_generation(&mut self) -> bool {
        self.calc_fitness();
        if self.pop.iter().any(|m| m.fitness == 1.0) {
            return false;
        }
        self.mate();
        self.reproduce();
        true
    }
}

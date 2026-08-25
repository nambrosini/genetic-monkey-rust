use rand::seq::IndexedRandom;

use crate::monkey::Monkey;

const MATING_POOL_SCALE: f64 = 100.0;

pub struct Population {
    pop_size: usize,
    mutation_rate: f64,
    pop: Vec<Monkey>,
    mating_pool: Vec<usize>,
    average_fitness: f64,
    target: String,
    generation: i64,
}

impl Population {
    pub fn new(target: &str, pop_size: usize, mutation_rate: f64) -> Self {
        assert!(pop_size != 0, "population must not be 0");
        assert!(!target.is_empty(), "target must not be empty");
        Self {
            pop_size,
            mutation_rate,
            pop: (0..pop_size).map(|_| Monkey::new(target.len())).collect(),
            mating_pool: vec![],
            average_fitness: 0f64,
            target: target.to_string(),
            generation: 0,
        }
    }

    fn calc_fitness(&mut self) {
        self.pop
            .iter_mut()
            .for_each(|m| m.calc_fitness(&self.target));
        let total_fitness = self.pop.iter().fold(0f64, |acc, m| acc + m.fitness());
        self.average_fitness = total_fitness / self.pop_size as f64;
    }

    fn mate(&mut self) {
        self.mating_pool.clear();
        for (index, monkey) in self.pop.iter().enumerate() {
            let n = ((monkey.fitness() * MATING_POOL_SCALE).ceil() as usize).max(1);
            for _ in 0..n {
                self.mating_pool.push(index);
            }
        }
    }

    fn reproduce(&mut self) {
        let mut rng = rand::rng();
        let mut pop = vec![];
        for _ in &self.pop {
            // mate() pushes at least one index per monkey in self.pop, and
            // Population::new asserts pop_size != 0, so mating_pool is never empty here.
            let a = &self.pop[*self.mating_pool.choose(&mut rng).unwrap()];
            let b = &self.pop[*self.mating_pool.choose(&mut rng).unwrap()];
            let mut child = a.crossover(b);
            child.mutate(self.mutation_rate);
            pop.push(child);
        }
        self.mating_pool.clear();
        self.pop = pop;
        self.pop.sort_by(|a, b| a.fitness().total_cmp(&b.fitness()));
    }

    pub fn get_average_fitness(&self) -> f64 {
        self.average_fitness
    }

    pub fn best_monkey(&self) -> String {
        self.pop
            .iter()
            .max_by(|a, b| a.fitness().total_cmp(&b.fitness()))
            .unwrap()
            .genes()
            .iter()
            .collect()
    }

    pub fn simulate_generation(&mut self) -> bool {
        self.calc_fitness();
        if self.pop.iter().any(|m| m.fitness() == 1.0) {
            return false;
        }
        self.mate();
        self.reproduce();
        self.generation += 1;
        true
    }

    pub fn has_ended(&self) -> bool {
        self.best_monkey() == self.target
    }

    pub fn generation(&self) -> i64 {
        self.generation
    }

    pub fn pop_size(&self) -> usize {
        self.pop_size
    }

    pub fn top_phrases(&self, num: usize) -> Vec<String> {
        self.pop
            .iter()
            .take(num)
            .map(|m| m.genes().iter().collect())
            .collect()
    }
}

impl Default for Population {
    fn default() -> Self {
        Self::new("to be or not to be", 1000, 0.01)
    }
}

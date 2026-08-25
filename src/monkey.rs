use std::ops::RangeInclusive;

use rand::{RngExt, rngs::ThreadRng};

const ASCII_PRINTABLE: RangeInclusive<u8> = 32..=126;

pub struct Monkey {
    genes: Vec<char>,
    fitness: f64,
}

impl Monkey {
    pub fn new(len: usize) -> Self {
        assert!(len != 0, "genes must be more than 0");
        let mut rng = rand::rng();
        Self {
            genes: (0..len).map(|_| generate_random_char(&mut rng)).collect(),
            fitness: 0f64,
        }
    }

    pub fn calc_fitness(&mut self, target: &str) {
        assert!(
            target.len() == self.genes.len(),
            "target length doesn't match genes length"
        );
        let score = self
            .genes
            .iter()
            .zip(target.chars())
            .filter(|(a, b)| a == &b)
            .count() as f64;

        self.fitness = score / target.len() as f64
    }

    pub fn crossover(&self, other: &Monkey) -> Monkey {
        let mut rng = rand::rng();
        let mut genes = self.genes.clone();
        let midpoint = rng.random_range(0..self.genes.len());
        genes[midpoint + 1..].copy_from_slice(&other.genes[midpoint + 1..]);
        Monkey {
            genes,
            fitness: 0.0,
        }
    }

    pub fn mutate(&mut self, mutation_rate: f64) {
        let mut rng = rand::rng();
        for g in self.genes.iter_mut() {
            if rng.random::<f64>() < mutation_rate {
                *g = generate_random_char(&mut rng)
            }
        }
    }

    pub fn genes(&self) -> &[char] {
        &self.genes
    }

    pub fn fitness(&self) -> f64 {
        self.fitness
    }
}

fn generate_random_char(rng: &mut ThreadRng) -> char {
    rng.random_range(ASCII_PRINTABLE) as char
}

#[cfg(test)]
mod tests {
    use crate::monkey::Monkey;

    #[test]
    fn test_calc_fitness_half() {
        let target = "Ambri";
        let mut monkey = Monkey {
            genes: "amhri".chars().collect(),
            fitness: 0.0,
        };
        monkey.calc_fitness(target);

        assert_eq!(monkey.fitness(), 0.6)
    }

    #[test]
    fn test_calc_fitness_no() {
        let target = "Ambri";
        let mut monkey = Monkey {
            genes: "rsaua".chars().collect(),
            fitness: 0.0,
        };
        monkey.calc_fitness(target);

        assert_eq!(monkey.fitness(), 0.0)
    }

    #[test]
    fn test_calc_fitness_full() {
        let target = "Ambri";
        let mut monkey = Monkey {
            genes: "Ambri".chars().collect(),
            fitness: 0.0,
        };
        monkey.calc_fitness(target);

        assert_eq!(monkey.fitness(), 1.0)
    }

    #[test]
    fn crossover() {
        let a = Monkey {
            genes: "aaaaa".chars().collect(),
            fitness: 0.0,
        };
        let b = Monkey {
            genes: "bbbbb".chars().collect(),
            fitness: 0.0,
        };

        let child = a.crossover(&b);

        for (i, gene) in child.genes.iter().enumerate() {
            assert!(gene == &a.genes[i] || gene == &b.genes[i])
        }
    }
}

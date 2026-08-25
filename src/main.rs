use genetic_monkey_rust::population::Population;

fn main() {
    let mut pop = Population::new("Forza Ambri!", 100, 0.01);
    let mut generation = 0;
    loop {
        generation += 1;
        if !pop.simulate_generation() {
            break;
        }
        println!(
            "Generation: {generation}, Average Fitness: {}, Best: {}",
            pop.get_average_fitness(),
            pop.best_monkey()
        )
    }

    println!("Generation: {generation}")
}

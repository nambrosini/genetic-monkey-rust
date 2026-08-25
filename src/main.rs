use genetic_monkey_rust::population::Population;
use iced::{
    Element,
    Length::Fill,
    Subscription, Theme,
    time::{self, milliseconds},
    widget::{button, column, container, row, text, text_input},
};

fn main() -> iced::Result {
    iced::application(Application::default, Application::update, Application::view)
        .subscription(Application::subscription)
        .run()
}

#[derive(Default)]
struct Application {
    population: Population,
    running: bool,
    settings: Settings,
}

struct Settings {
    target: String,
    pop_size: usize,
    mutation_rate: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            target: "to be or not to be".to_string(),
            pop_size: 100,
            mutation_rate: 1,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Start,
    Stop,
    Simulate,
    TargetChanged(String),
    PopulationSizeChanged(String),
    MutationChanged(String),
}

impl Application {
    fn update(&mut self, message: Message) {
        match message {
            Message::Start => {
                self.running = true;
                self.population = Population::new(
                    &self.settings.target,
                    self.settings.pop_size,
                    self.settings.mutation_rate as f64 / 100.0,
                )
            }
            Message::Stop => self.running = false,
            Message::Simulate => {
                if !self.running {
                    return;
                }
                self.population.simulate_generation();
                self.running = !self.population.has_ended();
            }
            Message::TargetChanged(target) => self.settings.target = target,
            Message::PopulationSizeChanged(pop_size) => {
                if let Ok(x) = pop_size.parse::<usize>() {
                    self.settings.pop_size = x;
                }
            }
            Message::MutationChanged(mutation) => {
                if let Ok(x) = mutation.parse::<usize>() {
                    self.settings.mutation_rate = x
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        container(column!(
            text("Infinite Monkey Theorem").size(50),
            text("Solved using a genetic algorithm.").size(25),
            row!(
                // Settings
                column!(
                    text("Settings").size(35),
                    text("Target Text"),
                    text_input("", &self.settings.target).on_input(Message::TargetChanged),
                    text("Population Size"),
                    text_input("0", &self.settings.pop_size.to_string())
                        .on_input(Message::PopulationSizeChanged),
                    text("Mutation Rate (%)"),
                    text_input("1", &self.settings.mutation_rate.to_string())
                        .on_input(Message::MutationChanged),
                    text("Show number of phrases"),
                    row!(
                        button("Start")
                            .on_press(Message::Start)
                            .style(|theme: &Theme, status| {
                                let palette = theme.extended_palette();

                                match status {
                                    button::Status::Active => button::Style::default()
                                        .with_background(palette.success.weak.color),
                                    _ => button::Style::default()
                                        .with_background(palette.success.strong.color),
                                }
                            })
                            .padding(10),
                        button("Stop")
                            .on_press(Message::Stop)
                            .style(|theme: &Theme, status| {
                                let palette = theme.extended_palette();

                                match status {
                                    button::Status::Active => button::Style::default()
                                        .with_background(palette.danger.weak.color),
                                    _ => button::Style::default()
                                        .with_background(palette.danger.strong.color),
                                }
                            }),
                    )
                    .padding(10),
                )
                .padding(10)
                .width(Fill)
                .height(Fill),
                // Best monkey
                column!(
                    text("Stats").size(35),
                    text!("Best: {}", self.population.best_monkey()),
                    text!("Generation # {}", self.population.generation()),
                    text!(
                        "Average Fitness: {:.2}",
                        self.population.get_average_fitness()
                    ),
                    text!("Population Size: {}", self.population.pop_size()),
                )
                .padding(10)
                .width(Fill)
                .height(Fill),
            )
            .width(Fill)
            .height(Fill),
        ))
        .width(Fill)
        .height(Fill)
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(milliseconds(20)).map(|_| Message::Simulate)
    }
}

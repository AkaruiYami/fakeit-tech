use colored::*;
use rand::Rng;
use rand::rngs::ThreadRng;

use crate::engine::FakeModule;

pub struct AiModule;

impl FakeModule for AiModule {
    fn name(&self) -> &str {
        "ai"
    }

    fn run(&self, rng: &mut ThreadRng) {
        let percent = rng.random_range(1..=100);

        println!("{}", format!("[AI] Training model: {}%", percent).blue());
    }
}

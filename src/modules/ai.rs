use std::thread;
use std::time::Duration;

use colored::*;
use rand::Rng;
use rand::rngs::ThreadRng;

use crate::engine::FakeModule;
use crate::modules::registry;

pub struct AiModule;

impl FakeModule for AiModule {
    fn name(&self) -> &str {
        "ai"
    }

    // TODO: Let the percentage properly sequential instead of jumble random
    fn run(&self, rng: &mut ThreadRng) {
        let percent = rng.random_range(1..=100);

        println!("{}", format!("[AI] Training model: {}%", percent).blue());
        let delay_ms = rng.random_range(50..=350);
        thread::sleep(Duration::from_millis(delay_ms));
    }
}

#[ctor::ctor]
fn register_build() {
    registry::register(Box::new(AiModule));
}

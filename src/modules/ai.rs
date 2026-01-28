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

    fn run(&self, rng: &mut ThreadRng) {
        let mut curr_percent = 0;

        while curr_percent < 100 {
            println!(
                "{}",
                format!("[AI] Training model: {}%", curr_percent).blue()
            );
            let delay_ms = rng.random_range(50..=350);
            thread::sleep(Duration::from_millis(delay_ms));
            if rng.random_range(0..=100) <= 50 {
                curr_percent += 1;
            }
        }
        println!("{}", format!("[AI] Training model: {}%", 100).green());
    }
}

#[ctor::ctor]
fn register_build() {
    registry::register(Box::new(AiModule));
}

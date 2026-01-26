use colored::*;
use rand::Rng;
use rand::rngs::ThreadRng;

use crate::engine::FakeModule;

pub struct HackModule;

impl FakeModule for HackModule {
    fn name(&self) -> &str {
        "hack"
    }

    fn run(&self, rng: &mut ThreadRng) {
        let ip = format!(
            "{}.{}.{}.{}",
            rng.random_range(1..255),
            rng.random_range(1..255),
            rng.random_range(1..255),
            rng.random_range(1..255),
        );

        println!("{}", format!("[HACK] Scanning {}", ip).red());
    }
}

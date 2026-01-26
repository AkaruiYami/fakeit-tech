#![allow(dead_code)]
use std::thread;
use std::time::Duration;

use colored::*;
use rand::Rng;
use rand::rngs::ThreadRng;

use crate::engine::FakeModule;
use crate::modules::registry;

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
        let delay_ms = rng.random_range(50..=350);
        thread::sleep(Duration::from_millis(delay_ms));
    }
}

#[ctor::ctor] // <-- run at compile-time before main
fn register_build() {
    registry::register(Box::new(HackModule));
}

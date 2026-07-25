use std::thread;
use std::time::Duration;

use colored::*;
use rand::Rng;

use crate::engine::{FakeModule, RunContext};
use crate::modules::registry;

pub struct HackModule;

impl FakeModule for HackModule {
    fn name(&self) -> &str {
        "hack"
    }

    fn run(&self, ctx: &mut RunContext) {
        let ip = format!(
            "{}.{}.{}.{}",
            ctx.rng.random_range(1..255),
            ctx.rng.random_range(1..255),
            ctx.rng.random_range(1..255),
            ctx.rng.random_range(1..255),
        );

        println!("{}", format!("[HACK] Scanning {}", ip).red());
        let delay_ms = ctx.rng.random_range(ctx.delay_min..=ctx.delay_max);
        thread::sleep(Duration::from_millis(delay_ms));
    }
}

#[ctor::ctor]
fn register_hack() {
    registry::register(Box::new(HackModule));
}

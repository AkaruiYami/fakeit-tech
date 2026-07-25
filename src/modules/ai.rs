use std::thread;
use std::time::Duration;

use colored::*;
use rand::Rng;

use crate::engine::{FakeModule, RunContext};
use crate::modules::registry;

pub struct AiModule;

impl FakeModule for AiModule {
    fn name(&self) -> &str {
        "ai"
    }

    fn run(&self, ctx: &mut RunContext) {
        let mut curr_percent = 0;

        while curr_percent < 100 {
            println!(
                "{}",
                format!("[AI] Training model: {}%", curr_percent).blue()
            );
            let delay_ms = ctx.rng.random_range(ctx.delay_min..=ctx.delay_max);
            thread::sleep(Duration::from_millis(delay_ms));
            if ctx.rng.random_range(0..=100) <= 50 {
                curr_percent += 1;
            }
        }
        println!("{}", format!("[AI] Training model: {}%", 100).green());
    }
}

#[ctor::ctor]
fn register_ai() {
    registry::register(Box::new(AiModule));
}

mod cli;
mod config;
mod config_loader;
mod engine;
mod modules;

use std::io::{self, Write};

use clap::Parser;
use engine::RunContext;
use modules::registry;

use crate::cli::Cli;

fn main() {
    let _ = ctrlc::set_handler(|| {
        print!("\x1B[?25h\x1B[0m");
        let _ = io::stdout().flush();
        std::process::exit(0);
    });

    let cmd = Cli::parse();

    if cmd.list {
        let registered_modules = registry::get_registered();
        println!("The following are modules that can be call:");
        for module in registered_modules {
            let name = module.name();
            println!("- {}", name);
        }
    } else {
        let mut rng = rand::rng();
        let mut active_modules = registry::get_registered();

        if active_modules.is_empty() {
            println!("No modules registered. Exiting.");
            return;
        }

        let config = config_loader::load_config();
        let (delay_min, delay_max, module_config) = match config {
            Some(c) => (c.core.delay_min, c.core.delay_max, Some(c.modules)),
            None => (200, 800, None),
        };

        let loop_mode = cmd._loop;
        let args_cleaned = cmd.modules;

        if !args_cleaned.is_empty() {
            active_modules.retain(|m| args_cleaned.contains(&m.name().to_string()));
        }

        active_modules.retain(|m| match &module_config {
            None => true,
            Some(mc) => {
                let enabled = match m.name() {
                    "build" => mc.build.unwrap_or(true),
                    "hack" => mc.hack.unwrap_or(true),
                    "ai" => mc.ai.unwrap_or(true),
                    "cypher-square" => mc.cypher_square.unwrap_or(true),
                    "matrix" => mc.matrix.unwrap_or(true),
                    _ => true,
                };
                enabled
            }
        });

        let mut ctx = RunContext {
            rng: &mut rng,
            delay_min,
            delay_max,
        };

        if loop_mode {
            loop {
                for module in &active_modules {
                    print!("\x1B[2J\x1B[1;1H");
                    module.run(&mut ctx);
                }
            }
        } else {
            for module in &active_modules {
                print!("\x1B[2J\x1B[1;1H");
                module.run(&mut ctx);
            }
        }
    }
}

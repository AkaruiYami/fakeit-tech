// mod config;
// mod config_loader;
mod cli;
mod engine;
mod modules;

use clap::Parser;
use modules::registry;

use crate::cli::Cli;

fn main() {
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

        // Fetch registered modules
        let mut active_modules = registry::get_registered();

        if active_modules.is_empty() {
            println!("No modules registered. Exiting.");
            return;
        }

        let loop_mode = cmd._loop;
        let args_cleaned = cmd.modules;

        if !args_cleaned.is_empty() {
            active_modules.retain(|m| args_cleaned.contains(&m.name().to_string()));
        }

        if loop_mode {
            loop {
                for module in &active_modules {
                    print!("\x1B[2J\x1B[1;1H");
                    module.run(&mut rng);
                }
            }
        } else {
            for module in &active_modules {
                print!("\x1B[2J\x1B[1;1H");
                module.run(&mut rng);
            }
        }
    }
}

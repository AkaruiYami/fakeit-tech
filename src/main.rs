// mod config;
// mod config_loader;
mod engine;
mod modules;

use modules::registry;

fn main() {
    let mut rng = rand::rng();

    // Fetch registered modules
    let mut active_modules = registry::get_registered();

    if active_modules.is_empty() {
        println!("No modules registered. Exiting.");
        return;
    }

    // Optional: filter by command-line args
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut loop_mode = false;
    let args_cleaned: Vec<String> = args
        .iter()
        .filter(|a| {
            if a == &"--loop" {
                loop_mode = true;
                false
            } else {
                true
            }
        })
        .cloned()
        .collect();

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

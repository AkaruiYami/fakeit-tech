mod config;
mod config_loader;
mod engine;
mod modules;

use rand::{Rng, rng};
use std::{thread, time::Duration};

use config_loader::load_config;
use modules::load_modules;

fn main() {
    let config = load_config();

    let mut rng = rng();

    let modules = load_modules(&config.modules);

    for module in &modules {
        module.run(&mut rng);

        let delay = rng.random_range(config.core.delay_min..config.core.delay_max);

        thread::sleep(Duration::from_millis(delay));
    }
}

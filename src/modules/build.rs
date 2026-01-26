use colored::*;
use rand::{rngs::ThreadRng, seq::IndexedRandom};

use crate::engine::FakeModule;

pub struct BuildModule;

impl FakeModule for BuildModule {
    fn name(&self) -> &str {
        "build"
    }

    fn run(&self, rng: &mut ThreadRng) {
        let steps = [
            "Compiling sources",
            "Linking binaries",
            "Optimizing assets",
            "Running tests",
            "Packaging release",
        ];

        let step = steps.choose(rng).unwrap();

        println!("{}", format!("[BUILD] {}", step).green());
    }
}

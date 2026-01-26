use colored::*;
use rand::Rng;
use rand::rngs::ThreadRng;
use rand::seq::IndexedRandom;
use std::{thread, time::Duration};

use crate::engine::FakeModule;

pub struct BuildModule;

impl FakeModule for BuildModule {
    fn name(&self) -> &str {
        "build"
    }

    fn run(&self, rng: &mut ThreadRng) {
        let steps = [
            "Collecting source files",
            "Loading assets",
            "Parsing config files",
            "Compiling module 'core'",
            "Compiling module 'utils'",
            "Processing textures",
            "Optimizing images",
            "Linking binaries",
            "Running unit tests",
            "Packaging release",
            "Generating documentation",
            "Finalizing build",
        ];

        let colors = [Color::Green, Color::Blue, Color::Magenta, Color::Cyan];

        let fake_username = "user01x";
        let roots = [
            format!("/home/{}/projects/app", fake_username),
            format!("/home/{}/dev/fakeit", fake_username),
            "/build/workspace".to_string(),
            "/tmp/build-cache".to_string(),
            "/opt/sdk/resources".to_string(),
        ];

        // Divide 0..100% evenly across steps
        let total_steps = steps.len();
        let mut current_percent = 0;

        for (i, step) in steps.iter().enumerate() {
            let color = colors.choose(rng).unwrap();

            // Calculate approximate progress range for this step
            let step_percent_start = current_percent;
            let step_percent_end = ((i + 1) * 100 / total_steps) as u8;

            // Resource-heavy steps
            if step.contains("Collecting")
                || step.contains("Loading")
                || step.contains("Processing")
            {
                let num_files = rng.random_range(7..55);
                for j in 0..num_files {
                    let path = generate_fake_path(rng, &roots);

                    // Progress increments within step
                    let percent = step_percent_start
                        + (((step_percent_end - step_percent_start) as usize * j) / num_files)
                            as u8;

                    println!(
                        "[{}%] {} -> {}",
                        percent,
                        step.color(*color),
                        path.bright_black()
                    );

                    let delay_ms = rng.random_range(50..=350);
                    thread::sleep(Duration::from_millis(delay_ms));
                }
            } else {
                let percent = step_percent_end;

                let chance: f32 = rng.random();
                if chance < 0.1 {
                    println!(
                        "{}",
                        format!("[WARN] {}: deprecated API detected", step).yellow()
                    );
                } else if chance < 0.15 {
                    println!(
                        "{}",
                        format!("[ERROR] {}: failed to resolve dependency", step).red()
                    );
                    let delay_ms = rng.random_range(500..=750);
                    thread::sleep(Duration::from_millis(delay_ms));
                } else {
                    println!("[{}%] {}", percent, step.color(*color));
                }

                // Simulate processing time
                let delay_ms = rng.random_range(300..=650);
                thread::sleep(Duration::from_millis(delay_ms));
            }

            current_percent = step_percent_end;
        }

        // Build complete
        println!("{}", "[100%] Build finished successfully!".green().bold());
    }
}

// Generate fake paths
fn generate_fake_path(rng: &mut ThreadRng, roots: &[String]) -> String {
    let folders = [
        "src", "core", "utils", "assets", "textures", "audio", "shaders", "config", "vendor",
        "modules", "cache",
    ];

    let names = [
        "main", "engine", "renderer", "audio", "input", "network", "ui", "physics", "math",
        "shader", "texture", "config",
    ];

    let exts = [
        "rs", "c", "cpp", "h", "json", "toml", "png", "jpg", "wav", "mp3", "vert", "frag",
    ];

    let root = roots.choose(rng).unwrap();

    // Random depth
    let depth = rng.random_range(2..=5);
    let mut path = root.clone();

    for _ in 0..depth {
        let folder = folders.choose(rng).unwrap();
        path.push('/');
        path.push_str(folder);
    }

    let name = names.choose(rng).unwrap();
    let ext = exts.choose(rng).unwrap();
    let id: u16 = rng.random_range(1..=999);

    format!("{}/{}_{}.{}", path, name, id, ext)
}

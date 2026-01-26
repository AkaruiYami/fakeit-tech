use colored::*;
use rand::Rng;
use rand::rngs::ThreadRng;
use rand::seq::IndexedRandom;

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

        let colors = [
            Color::Red,
            Color::Green,
            Color::Yellow,
            Color::Blue,
            Color::Magenta,
            Color::Cyan,
        ];

        let num_steps = rng.random_range(1..=2);
        let chosen_steps: Vec<_> = steps.choose_multiple(rng, num_steps).collect();

        for step in chosen_steps {
            let color = colors.choose(rng).unwrap();
            let percent: u8 = rng.random_range(1..=100);

            // Resource-heavy steps
            if step.contains("Collecting")
                || step.contains("Loading")
                || step.contains("Processing")
            {
                let count = rng.random_range(6..15);

                for _ in 0..count {
                    let path = generate_fake_path(rng);

                    println!(
                        "[{}%] {} -> {}",
                        percent,
                        step.color(*color),
                        path.bright_black()
                    );
                }
            } else {
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
                } else {
                    println!("[{}%] {}", percent, step.color(*color));
                }
            }
        }
    }
}

fn generate_fake_path(rng: &mut ThreadRng) -> String {
    let fake_username = "user01x";
    let roots = [
        format!("/home/{}/projects/app", fake_username),
        format!("/home/{}/dev/fakeit", fake_username),
        "/build/workspace".to_string(),
        "/tmp/build-cache".to_string(),
        "/opt/sdk/resources".to_string(),
    ];

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

    // Random depth (2–4 folders)
    let depth = rng.random_range(2..=4);

    let mut path = String::from(root);

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

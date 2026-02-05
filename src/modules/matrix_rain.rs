#![allow(dead_code)]

use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use rand::Rng;
use rand::rngs::ThreadRng;

use crossterm::terminal;

use crate::engine::FakeModule;
use crate::modules::registry;

pub struct MatrixModule;

impl FakeModule for MatrixModule {
    fn name(&self) -> &str {
        "matrix"
    }

    fn run(&self, rng: &mut ThreadRng) {
        let _ = ctrlc::set_handler(|| {
            print!("\x1B[?25h\x1B[0m");
            let _ = io::stdout().flush();
            std::process::exit(0);
        });

        let (width, height) = match terminal::size() {
            Ok((w, h)) => (w as usize, h as usize),
            Err(_) => (80, 24),
        };

        let frames = 150;

        let mut drops: Vec<usize> = (0..width).map(|_| rng.random_range(0..height)).collect();

        // Clear screen & hide cursor
        print!("\x1b[2J\x1b[?25l");
        io::stdout().flush().ok();

        for _ in 0..frames {
            print!("\x1b[H");

            for y in 0..height {
                (0..width).for_each(|x| {
                    if drops[x] == y {
                        let c = rng.random_range(33u8..127u8) as char;
                        print!("\x1b[92m{}\x1b[0m", c);
                    } else if drops[x].saturating_sub(1) == y {
                        let c = rng.random_range(33u8..127u8) as char;
                        print!("\x1b[32m{}\x1b[0m", c);
                    } else {
                        print!(" ");
                    }
                });
                println!();
            }

            io::stdout().flush().ok();

            (0..width).for_each(|x| {
                if drops[x] > height {
                    if rng.random_bool(0.08) {
                        drops[x] = 0;
                    }
                } else {
                    drops[x] += 1;
                }
            });

            thread::sleep(Duration::from_millis(45));
        }

        // Restore cursor
        print!("\x1b[?25h");
        io::stdout().flush().ok();
    }
}

#[ctor::ctor]
fn register_build() {
    registry::register(Box::new(MatrixModule));
}

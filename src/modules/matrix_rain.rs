use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use rand::Rng;
use rand::rngs::ThreadRng;

use crate::engine::FakeModule;
use crate::modules::registry;

pub struct MatrixModule;

impl FakeModule for MatrixModule {
    fn name(&self) -> &str {
        "matrix"
    }

    fn run(&self, rng: &mut ThreadRng) {
        let (width, height) = match crossterm::terminal::size() {
            Ok((w, h)) => (
                (w as usize).saturating_sub(1),
                (h as usize).saturating_sub(1),
            ),
            Err(_) => (79, 23),
        };

        let frames = 150;

        let mut drops: Vec<usize> = (0..width).map(|_| rng.random_range(0..height)).collect();

        print!("\x1B[2J\x1B[3J\x1B[H");
        print!("\x1b[?25l");
        io::stdout().flush().ok();

        for _ in 0..frames {
            let mut frame = String::with_capacity(width * height * 3);
            frame.push_str("\x1b[H");

            for y in 0..height {
                for x in 0..width {
                    if drops[x] == y {
                        let c = rng.random_range(33u8..127u8) as char;
                        frame.push_str(&format!("\x1b[92m{}\x1b[0m", c));
                    } else if drops[x].saturating_sub(1) == y {
                        let c = rng.random_range(33u8..127u8) as char;
                        frame.push_str(&format!("\x1b[32m{}\x1b[0m", c));
                    } else {
                        frame.push(' ');
                    }
                }
                if y + 1 < height {
                    frame.push('\n');
                }
            }

            print!("{}", frame);
            io::stdout().flush().ok();

            for x in 0..width {
                if drops[x] > height {
                    if rng.random_bool(0.08) {
                        drops[x] = 0;
                    }
                } else {
                    drops[x] += 1;
                }
            }

            thread::sleep(Duration::from_millis(45));
        }

        print!("\x1b[?25h");
        io::stdout().flush().ok();
    }
}

#[ctor::ctor]
fn register_matrix() {
    registry::register(Box::new(MatrixModule));
}

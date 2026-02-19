use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use colored::*;
use crossterm::terminal;
use rand::Rng;
use rand::rngs::ThreadRng;

use crate::engine::FakeModule;
use crate::modules::registry;

pub struct CypherSquare;

impl FakeModule for CypherSquare {
    fn name(&self) -> &str {
        "cypher-square"
    }

    fn run(&self, rng: &mut ThreadRng) {
        let _ = ctrlc::set_handler(|| {
            print!("\x1B[?25h\x1B[0m");
            let _ = io::stdout().flush();
            std::process::exit(0);
        });

        let mutate_delay = 200; // ms
        let error_chance = 0.07;

        let (width, height) = match terminal::size() {
            Ok((w, h)) => (w as usize, h as usize),
            Err(_) => (80, 24),
        };

        print!("\x1B[?25l");
        io::stdout().flush().unwrap();

        let mut grid: Vec<Vec<char>> = (0..height)
            .map(|_| (0..width).map(|_| random_char(rng)).collect())
            .collect();

        loop {
            print!("\x1B[2J\x1B[1;1H");

            for row in &grid {
                for c in row {
                    if rng.random_bool(error_chance) {
                        print!("{} ", c.to_string().red());
                    } else {
                        print!("{} ", c.to_string().green());
                    }
                }
                println!();
            }

            print!("\x1B[0m");

            for _ in 0..8 {
                let x = rng.random_range(0..width);
                let y = rng.random_range(0..height);

                grid[y][x] = random_char(rng);
            }

            io::stdout().flush().unwrap();

            thread::sleep(Duration::from_millis(mutate_delay));
        }
    }
}

#[ctor::ctor]
fn register_build() {
    registry::register(Box::new(CypherSquare));
}

fn random_char(rng: &mut ThreadRng) -> char {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
          abcdefghijklmnopqrstuvwxyz\
          0123456789\
          !@#$%^&*()-_=+[]{}|;:,.<>?/";

    let i = rng.random_range(0..CHARSET.len());
    CHARSET[i] as char
}

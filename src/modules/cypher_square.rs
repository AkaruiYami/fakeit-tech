use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use rand::Rng;
use rand::rngs::ThreadRng;

use crate::engine::{FakeModule, RunContext};
use crate::modules::registry;

pub struct CypherSquare;

impl FakeModule for CypherSquare {
    fn name(&self) -> &str {
        "cypher-square"
    }

    fn run(&self, ctx: &mut RunContext) {
        let error_chance = 0.07;

        let (width, height) = match crossterm::terminal::size() {
            Ok((w, h)) => (
                (w as usize).saturating_sub(1) / 2,
                (h as usize).saturating_sub(1),
            ),
            Err(_) => (39, 23),
        };

        print!("\x1B[2J\x1B[3J\x1B[H");
        print!("\x1B[?25l");
        io::stdout().flush().unwrap();

        let mut grid: Vec<Vec<char>> = (0..height)
            .map(|_| (0..width).map(|_| random_char(ctx.rng)).collect())
            .collect();

        let frames = 150;

        for _ in 0..frames {
            let mut frame = String::with_capacity(width * height * 8);
            frame.push_str("\x1B[H");

            for (i, row) in grid.iter().enumerate() {
                for (j, c) in row.iter().enumerate() {
                    if j > 0 {
                        frame.push(' ');
                    }
                    if ctx.rng.random_bool(error_chance) {
                        frame.push_str("\x1B[31m");
                    } else {
                        frame.push_str("\x1B[32m");
                    }
                    frame.push(*c);
                    frame.push_str("\x1B[0m");
                }
                if i + 1 < height {
                    frame.push('\n');
                }
            }

            print!("{}", frame);

            for _ in 0..8 {
                let x = ctx.rng.random_range(0..width);
                let y = ctx.rng.random_range(0..height);
                grid[y][x] = random_char(ctx.rng);
            }

            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(ctx.delay_min));
        }

        print!("\x1b[?25h");
        io::stdout().flush().ok();
    }
}

#[ctor::ctor]
fn register_cypher_square() {
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

#![allow(dead_code)]
use rand::rngs::ThreadRng;

pub trait FakeModule {
    fn name(&self) -> &str;

    fn run(&self, rng: &mut ThreadRng);
}

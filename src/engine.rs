use rand::rngs::ThreadRng;

pub struct RunContext<'a> {
    pub rng: &'a mut ThreadRng,
    pub delay_min: u64,
    pub delay_max: u64,
}

pub trait FakeModule {
    fn name(&self) -> &str;

    fn run(&self, ctx: &mut RunContext);
}

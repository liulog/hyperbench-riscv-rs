use super::Benchmark;

pub struct Idle;

impl Benchmark for Idle {
    fn init(&self) {}

    fn benchmark(&self) {}

    fn clean(&self) {}
}

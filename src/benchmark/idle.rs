use super::Benchmark;

pub struct Idle;

pub const IDLE: usize = 1;

impl Benchmark for Idle {
    fn init(&self) {}

    fn benchmark(&self) {
        for _ in 0..IDLE {}
    }

    fn clean(&self) {}
}

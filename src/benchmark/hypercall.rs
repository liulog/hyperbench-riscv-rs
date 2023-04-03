use crate::sbi::sbi_call;

use super::Benchmark;

pub struct Hypercall;

pub const HYPERCALL_COUNT: usize = 1000;

impl Benchmark for Hypercall {
    fn init(&self) {}

    fn benchmark_control(&self) {
        for _ in 0..HYPERCALL_COUNT {}
    }

    fn benchmark(&self) {
        for _ in 0..HYPERCALL_COUNT {
            sbi_call(usize::MAX, 0, 0, 0);
        }
    }

    fn clean(&self) {}
}

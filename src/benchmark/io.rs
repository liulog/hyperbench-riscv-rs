use super::Benchmark;

pub struct In;

impl Benchmark for In {
    fn init(&self) {}

    fn benchmark(&self) {}

    fn clean(&self) {}
}

pub struct Out;

impl Benchmark for Out {
    fn init(&self) {}

    fn benchmark(&self) {}

    fn clean(&self) {}
}

pub struct Print;

impl Benchmark for Print {
    fn init(&self) {}

    fn benchmark(&self) {}

    fn clean(&self) {}
}

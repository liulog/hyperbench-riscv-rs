use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

pub mod hypercall;
pub mod idle;
pub mod io;
pub mod memory;

use hypercall::Hypercall;
use idle::Idle;
use io::{In, Out, Print};
use memory::{ColdMemoryAccess, HotMemoryAccess, SetPageTable};

pub trait Benchmark {
    /// initialize benchmark context
    fn init(&self);
    /// execute benchmark function call
    fn benchmark(&self);
    /// clean benchmark context
    fn clean(&self);
}

pub struct BenchmarkTable {
    pub table: Vec<Box<dyn Benchmark>>,
}

impl BenchmarkTable {
    pub fn init() -> Self {
        let table: Vec<Box<dyn Benchmark>> = vec![
            // idle
            Box::new(Idle),
            // hypercall
            Box::new(Hypercall),
            // memory benchmark
            Box::new(HotMemoryAccess),
            Box::new(ColdMemoryAccess),
            Box::new(SetPageTable),
            // IO benchmark
            Box::new(In),
            Box::new(Out),
            Box::new(Print),
        ];
        Self { table }
    }

    pub fn benchmark(&self) {
        for item in self.table.iter() {
            item.init();
            item.benchmark();
            item.clean();
        }
    }
}

use alloc::vec::Vec;
use alloc::vec;
use alloc::boxed::Box;



pub mod idle;
pub mod hypercall;
pub mod memory;
pub mod io;

use idle::Idle;
use hypercall::Hypercall;
use memory::{HotMemoryAccess, ColdMemoryAccess, SetPageTable};
use io::{In, Out, Print};

// pub struct Benchmark<F> {
//     pub name: String,
//     pub category: String,
//     pub init: Option<F>,
//     pub benchmark: F,
//     pub benchmark_contol: F,
//     pub cleanup: Option<F>,
//     pub iter_count: usize,
// }

pub trait Benchmark{
    /// initialize benchmark context
    fn init(&self);
    /// execute benchmark function call
    fn benchmark(&self);
    /// clean benchmark context
    fn clean(&self);
}

pub struct BenchmarkTable {
    pub table: Vec<Box<dyn Benchmark>>
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
            Box::new(Print)
        ];
        Self { table }
    }
}

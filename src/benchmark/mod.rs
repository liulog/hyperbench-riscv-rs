use alloc::vec;
use alloc::vec::Vec;
use alloc::{boxed::Box, string::String};

pub mod hypercall;
pub mod idle;
pub mod io;
pub mod memory;

use hypercall::Hypercall;
use idle::Idle;
use io::{Out, Print};
// use memory::{ColdMemoryAccess, HotMemoryAccess, SetPageTable};
use memory::{ColdMemoryAccess, HotMemoryAccess, SetPageTable};

use crate::{clint::read_mtime, println};

pub trait Benchmark {
    /// initialize benchmark context
    fn init(&self);
    /// execute benchmark function call
    fn benchmark(&self);
    /// The benchmark_control points to an idle loop function
    /// whose iteration count is the same as the current benchmark. It is used to offset the runtime of
    /// loop statements which must not be counted into the runtime of each benchmark.
    fn benchmark_control(&self);
    /// clean benchmark context
    fn clean(&self);
}

pub struct BenchmarkTable {
    pub table: Vec<(String, Box<dyn Benchmark>)>,
}

impl BenchmarkTable {
    pub fn init() -> Self {
        let table: Vec<(String, Box<dyn Benchmark>)> = vec![
            // idle
            (String::from("Idle"), Box::new(Idle)),
            // hypercall
            (String::from("Hypercall"), Box::new(Hypercall)),
            // memory benchmark
            (String::from("SetPageTable"), Box::new(SetPageTable)),
            (String::from("HotMemoryAccess"), Box::new(HotMemoryAccess)),
            (String::from("ColdMemoryAccess"), Box::new(ColdMemoryAccess)),
            // IO benchmark
            // (String::from("In"), Box::new(In)),
            (String::from("Out"), Box::new(Out)),
            (String::from("Print"), Box::new(Print)),
        ];
        Self { table }
    }

    /// # Safety
    pub unsafe fn benchmark(&self) {
        for (name, bench) in self.table.iter() {
            println!("============================");
            println!("Benchmark {}:", name);
            bench.init();
            let control_start = read_mtime();
            bench.benchmark_control();
            let control_end = read_mtime();
            let start_timer = read_mtime();
            bench.benchmark();
            let end_timer = read_mtime();
            bench.clean();
            let control_consume = control_end - control_start;
            let bench_consume = end_timer - start_timer;
            let actual_consume = bench_consume - control_consume;
            println!(
                "control start: {}, end: {}, consume: {}",
                control_start, control_end, control_consume
            );
            println!(
                "start: {}, end: {}, consume: {}",
                start_timer, end_timer, bench_consume
            );
            println!("actual consume: {}", actual_consume);
            println!("============================");
        }
    }
}

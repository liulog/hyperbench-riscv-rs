use alloc::vec;
use alloc::vec::Vec;
use alloc::{boxed::Box, string::String};

pub mod hypercall;
pub mod idle;
pub mod io;
pub mod memory;

use hypercall::Hypercall;
use idle::Idle;
use io::{In, Out, Print};
use memory::{ColdMemoryAccess, HotMemoryAccess, SetPageTable};

use crate::{clint::read_mtime, println};

pub trait Benchmark {
    /// initialize benchmark context
    fn init(&self);
    /// execute benchmark function call
    fn benchmark(&self);
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
            (String::from("HotMemoryAccess"), Box::new(HotMemoryAccess)),
            (String::from("ColdMemoryAccess"), Box::new(ColdMemoryAccess)),
            (String::from("SetPageTable"), Box::new(SetPageTable)),
            // IO benchmark
            (String::from("In"), Box::new(In)),
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
            let start_timer = read_mtime();
            bench.benchmark();
            let end_timer = read_mtime();
            bench.clean();
            println!(
                "start: {}, end: {}, consume: {}",
                start_timer,
                end_timer,
                end_timer - start_timer
            );
            println!("============================");
        }
    }
}

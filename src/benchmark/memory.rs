use crate::page_table::set_mmu;

use super::Benchmark;

pub struct HotMemoryAccess;

impl Benchmark for HotMemoryAccess {
    fn init(&self) {}

    fn benchmark_control(&self) {}

    fn benchmark(&self) {}

    fn clean(&self) {}
}

pub struct ColdMemoryAccess;

impl Benchmark for ColdMemoryAccess {
    fn init(&self) {}

    fn benchmark_control(&self) {}

    fn benchmark(&self) {}

    fn clean(&self) {}
}

pub struct SetPageTable;

impl Benchmark for SetPageTable {
    fn init(&self) {}

    fn benchmark_control(&self) {}

    fn benchmark(&self) {
        set_mmu(0x8000_0000, 128 * 1024 * 1024);
    }

    fn clean(&self) {}
}

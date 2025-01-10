use crate::{constants::PAGE_SIZE, page_table::set_mmu};

use super::Benchmark;

pub const HOT_MEMORY_COUNT: usize = 100000;
pub struct HotMemoryAccess;

impl Benchmark for HotMemoryAccess {
    fn init(&self) {}

    fn benchmark_control(&self) {
        let max_page_size = 128 * 1024 * 1024;
        for offset in 0..HOT_MEMORY_COUNT {
            let _: usize = 0x8000_0000 + (offset * 4 % max_page_size);
        }
    }

    fn benchmark(&self) {
        let max_page_size = 128 * 1024 * 1024;
        for offset in 0..HOT_MEMORY_COUNT {
            let addr: usize = 0x8000_0000 + (offset * 4 % max_page_size);
            unsafe { core::ptr::read(addr as *const ()) };
        }
    }

    fn clean(&self) {}
}

pub const COLD_MEMORY_COUNT: usize = 100000;

pub struct ColdMemoryAccess;

impl Benchmark for ColdMemoryAccess {
    fn init(&self) {}

    fn benchmark_control(&self) {
        let max_page = (128 * 1024 * 1024) / PAGE_SIZE;
        for i in 0..COLD_MEMORY_COUNT {
            let _ = 0x8000_0000 + (i % max_page) * PAGE_SIZE;
        }
    }

    fn benchmark(&self) {
        let max_page = (128 * 1024 * 1024) / PAGE_SIZE;
        for i in 0..COLD_MEMORY_COUNT {
            let addr = 0x8000_0000 + ((i % max_page) * PAGE_SIZE);
            unsafe { core::ptr::read(addr as *const ()) };
        }
    }

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

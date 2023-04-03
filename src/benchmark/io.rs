use crate::print;
use crate::sbi::console_putchar;

use super::Benchmark;

pub struct In;

impl Benchmark for In {
    fn init(&self) {}

    fn benchmark(&self) {}

    fn clean(&self) {}
}

pub const OUT_COUNT: usize = 1000;
pub struct Out;

impl Benchmark for Out {
    fn init(&self) {}

    fn benchmark(&self) {
        for _ in 0..OUT_COUNT {
            console_putchar('x' as usize);
        }
        print!("\n");
    }

    fn clean(&self) {}
}

pub const PRINT_COUNT: usize = 1000;
pub struct Print;

impl Benchmark for Print {
    fn init(&self) {}

    fn benchmark(&self) {
        let buf = "xxxxxxxxxxxxxxxxxxxxxxxx";
        for _ in 0..PRINT_COUNT {
            print!("{}", buf);
        }
        print!("\n");
    }

    fn clean(&self) {}
}

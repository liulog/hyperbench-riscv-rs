#![no_std]
#![no_main]
#![deny(warnings)]
#![allow(dead_code, non_upper_case_globals)]
#![feature(
    panic_info_message,
    alloc_error_handler,
    core_intrinsics,
    naked_functions,
    asm_const,
    stdsimd
)]

extern crate alloc;

mod benchmark;
#[macro_use]
mod console;
mod allocator;
mod constants;
mod lang_items;
mod page_table;
mod sbi;

pub const PAGE_SIZE: usize = 4096;

/// boot stack size
const BOOT_STACK_SIZE: usize = 16 * PAGE_SIZE;

#[link_section = ".bss.stack"]
/// boot stack
static BOOT_STACK: [u8; BOOT_STACK_SIZE] = [0u8; BOOT_STACK_SIZE];

#[link_section = ".text.entry"]
#[export_name = "_start"]
#[naked]
/// hypervisor entrypoint
pub unsafe extern "C" fn start() -> ! {
    core::arch::asm!(
        // prepare stack
        "la sp, {boot_stack}",
        "li t2, {boot_stack_size}",
        "addi t3, a0, 1",
        "mul t2, t2, t3",
        "add sp, sp, t2",
        // enter hentry
        "call benchmark_entry",
        boot_stack = sym BOOT_STACK,
        boot_stack_size = const BOOT_STACK_SIZE,
        options(noreturn)
    )
}

/// clear BSS segment
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0);
    }
}

#[no_mangle]
fn benchmark_entry() {
    clear_bss();
    println!("Hello World");
}

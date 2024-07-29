#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;

#[macro_use]
mod console;

mod lang_items;
mod sbi;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello, world!");
    extern "C" {
        fn s_text();
        fn e_text();
        fn s_data();
        fn e_data();
    }

    log!(info, ".text [{:#x}, {:#x}]", s_text as usize, e_text as usize);
    log!(debug, ".rodata [{:#x}, {:#x}]", s_data as usize, e_data as usize);
    log!(error, "test error");

    panic!("shut down!");
}

fn clear_bss() {
    extern  "C" {
        fn sbss();
        fn ebss();
    }

    (sbss as usize .. ebss as usize).for_each(|a| {
        unsafe{(a as *mut u8).write_volatile(0)}
    });
}
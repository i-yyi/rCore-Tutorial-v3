#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;


mod lang_items;
mod sbi;

#[macro_use]
mod log;
#[macro_use]
mod console;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    extern "C" {
        fn s_text();
        fn e_text();
        fn s_data();
        fn e_data();
    }

    INFO!(".text [{:#x}, {:#x}]", s_text as usize, e_text as usize);
    INFO!(".rodata [{:#x}, {:#x}]", s_data as usize, e_data as usize);
    INFO!("test error {}", 1);

    panic!("shutdown!");
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
#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![allow(unused)]

extern crate alloc;

#[path = "board/qemu.rs"]
mod board;

#[macro_use]
mod console;
mod lang_items;
mod logo;
mod mem;
pub mod mem_layout;
mod proc;
mod sbi;
mod sync;
pub mod syscall;
mod task;
mod trap;

use core::arch::global_asm;

use sbi::shutdown;

global_asm!(include_str!("entry.S"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn main() {
    clear_bss();
    logo::print_logo();
    trap::init();
    mem::init();
    shutdown()
}

// init .bss segment.
fn clear_bss() {
    // import external symbols.
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|x| unsafe { (x as *mut u8).write_volatile(0) });
}

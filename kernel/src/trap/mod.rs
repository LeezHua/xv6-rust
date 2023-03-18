use crate::{
    mem_layout::TRAMPOLINE,
    task::{current_pagetable, current_user_epc, current_user_trapcontext},
};
use core::arch::{asm, global_asm};

use riscv::register::{
    scause::{self, Exception, Interrupt, Trap},
    sepc,
    sstatus::{self, SPP},
    stval, stvec,
    utvec::TrapMode,
};

use crate::{
    syscall::syscall,
    task::{current_user_satp, run_next_task_kill, run_next_task_suspend},
    trap::interrupt::set_next_clock_interrupt,
};

pub mod context;
mod interrupt;
global_asm!(include_str!("trampoline.S"));

extern "C" {
    fn trampoline();
    fn user_trap();
    fn user_return();
}

pub fn init() {
    set_user_trap();
    interrupt::enable_clock_interrupt();
    set_next_clock_interrupt();
}

pub fn set_user_trap() {
    unsafe {
        stvec::write(
            TRAMPOLINE + (user_trap as usize - trampoline as usize),
            TrapMode::Direct,
        );
    }
}

pub fn set_kernel_trap() {
    unsafe {
        stvec::write(kernel_trap as usize, TrapMode::Direct);
    }
}

#[no_mangle]
pub fn user_trap_handler() {
    set_kernel_trap();

    if sstatus::read().spp() != SPP::User {
        panic!("user_trap_handler: not from user mode");
    }
    let cx = current_user_trapcontext();

    cx.epc = sepc::read();
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.epc += 4;
            let res = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
            cx.x[10] = res;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            println!("[kernel] PageFault in application, kernel killed it.");
            run_next_task_kill()
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            run_next_task_kill()
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            set_next_clock_interrupt();
            run_next_task_suspend();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}, sepc = {:#x}!",
                scause.cause(),
                stval,
                riscv::register::sepc::read()
            );
        }
    }

    user_trap_return()
}

pub fn user_trap_return() {
    unsafe {
        sstatus::set_spp(SPP::User);
    }
    let satp = current_user_satp();
    sepc::write(current_user_epc());
    set_user_trap();

    unsafe {
        asm! {
            "jr {0}",
            in(reg) TRAMPOLINE + (user_return as usize - trampoline as usize),
            in("a0") satp,
        }
    }
}

pub use context::TrapContext;

use self::interrupt::{enable_clock_interrupt, unable_clock_interrupt};

#[no_mangle]
pub fn kernel_trap() -> ! {
    panic!("kernel trap!");
}

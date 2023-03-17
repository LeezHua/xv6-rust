use core::arch::asm;
use lazy_static::*;

use riscv::register::satp;

use crate::{
    mem_layout::{KERNEL_BASE, PAGE_SIZE, PHYS_TOP, TRAMPOLINE},
    sync::UPSafeCell,
    task::param::MAX_APP_NUM,
};

use super::{
    address::Addr,
    kernel_stack_i,
    page_allocator::kalloc,
    page_table::{PTEFlags, PageTable},
};

extern "C" {
    fn etext();
    fn trampoline();
}

struct KernelSpace {
    page_table: PageTable,
}

impl KernelSpace {
    pub fn new() -> Self {
        Self {
            page_table: PageTable::new(),
        }
    }

    // 为每一个应用程序分配一个内核栈
    fn alloc_kernel_stack(&mut self) {
        for i in 0..MAX_APP_NUM {
            let page_tracker = kalloc().unwrap();
            let pa: Addr = page_tracker.page().into();
            self.page_table.push_page(page_tracker);
            self.page_table
                .map_range(kernel_stack_i(i), pa, PAGE_SIZE, PTEFlags::R | PTEFlags::W);
        }
    }

    pub fn init(&mut self) {
        self.page_table.map_range(
            Addr::new(KERNEL_BASE),
            Addr::new(KERNEL_BASE),
            etext as usize - KERNEL_BASE,
            PTEFlags::R | PTEFlags::X,
        );

        self.page_table.map_range(
            Addr::new(etext as usize),
            Addr::new(etext as usize),
            PHYS_TOP - etext as usize,
            PTEFlags::R | PTEFlags::W,
        );

        self.page_table.map_range(
            Addr::new(TRAMPOLINE),
            Addr::new(trampoline as usize),
            PAGE_SIZE,
            PTEFlags::R | PTEFlags::X,
        );

        self.alloc_kernel_stack();
    }
    pub fn print_kernel_pagetable(&self) {
        self.page_table.print_page_table();
    }
    pub fn active(&self) {
        unsafe {
            asm!("sfence.vma");
            satp::write(self.page_table.make_satp());
            asm!("sfence.vma");
        }
    }
    pub fn space_test(&self) {
        self.page_table.walk_test();
    }
}

lazy_static! {
    static ref KERNEL_SPACE: UPSafeCell<KernelSpace> =
        unsafe { UPSafeCell::new(KernelSpace::new()) };
}

pub fn kvminit() {
    let mut kernel_space = KERNEL_SPACE.get_mut();
    kernel_space.init();
    kernel_space.active();
    println!("kvminit success");
}

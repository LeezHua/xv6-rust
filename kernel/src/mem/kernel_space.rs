use alloc::collections::BTreeMap;
use core::arch::asm;
use lazy_static::*;

use riscv::register::satp;

use crate::{
    mem_layout::{KERNEL_BASE, KERNEL_STACK_SIZE, PAGE_SIZE, PHYS_TOP, TRAMPOLINE},
    sync::UPSafeCell,
    task::param::MAX_APP_NUM,
};

use super::{
    address::Addr,
    page_allocator::{kalloc, PageTracker},
    page_table::{PTEFlags, PageTable},
};

extern "C" {
    fn etext();
    fn trampoline();
}

pub struct KernelSpace {
    page_table: PageTable,
    data_pages: BTreeMap<Addr, PageTracker>,
}

impl KernelSpace {
    pub fn new() -> Self {
        Self {
            page_table: PageTable::new(),
            data_pages: BTreeMap::new(),
        }
    }

    // 为每一个应用程序分配一个内核栈
    fn alloc_kernel_stack(&mut self) {
        for i in 0..MAX_APP_NUM {
            let mut a = 0usize;
            while (a < KERNEL_STACK_SIZE) {
                let page_tracker = kalloc().unwrap();
                let pa: Addr = page_tracker.page().into();
                self.data_pages.insert(pa, page_tracker);

                self.page_table
                    .map(kernel_stack_i(i).add(a), pa, PTEFlags::R | PTEFlags::W);
                a += PAGE_SIZE;
            }
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

        self.page_table.map(
            Addr::new(TRAMPOLINE),
            Addr::new(trampoline as usize),
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

    pub fn make_satp(&self) -> usize {
        self.page_table.make_satp()
    }

    pub fn space_test(&self) {
        self.page_table.walk_test();
    }
}

lazy_static! {
    pub static ref KERNEL_SPACE: UPSafeCell<KernelSpace> =
        unsafe { UPSafeCell::new(KernelSpace::new()) };
}

pub fn kernel_stack_i(id: usize) -> Addr {
    Addr::new(TRAMPOLINE - (id + 1) * (KERNEL_STACK_SIZE + PAGE_SIZE))
}

pub fn kvminit() {
    let mut kernel_space = KERNEL_SPACE.get_mut();
    kernel_space.init();
    kernel_space.active();
    println!("[kernel] kernel space init success.");
}

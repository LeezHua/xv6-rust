use core::arch::asm;
use lazy_static::*;

use riscv::register::satp;

use crate::sync::UPSafeCell;

use super::{
    address::Addr,
    page_table::{PTEFlags, PageTable},
    param::{KERNEL_BASE, PHYS_TOP},
};

extern "C" {
    fn etext();
}

struct KernelSpace {
    page_table: PageTable,
}

impl KernelSpace {
    pub fn empty() -> Self {
        Self {
            page_table: PageTable::empty(),
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
        )
    }
    pub fn print_kernel_page_table(&self) {
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
        unsafe { UPSafeCell::new(KernelSpace::empty()) };
}

pub fn kvminit() {
    let mut kernel_space = KERNEL_SPACE.get_mut();
    kernel_space.init();
    kernel_space.active();
    kernel_space.space_test();
}

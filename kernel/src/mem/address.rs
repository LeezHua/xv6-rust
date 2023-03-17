use core::fmt::{self, Debug, Formatter};

use super::page_table::PageTableEntry;
use crate::mem_layout::{PAGE_BITS, PAGE_SIZE, PTE_FLAGS_BITS, PTE_NUM_PER_PAGE, PTE_NUM_PER_PAGE_BITS};

// 地址结构
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct Addr {
    pub bits: usize,
}

impl Debug for Addr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Addr: {:#x}", self.bits))
    }
}

impl<T> From<*mut T> for Addr {
    fn from(ptr: *mut T) -> Self {
        Self { bits: ptr as usize }
    }
}
impl<T> Into<*mut T> for Addr {
    fn into(self) -> *mut T {
        self.bits as *mut T
    }
}

impl Addr {
    pub fn empty() -> Self {
        Addr { bits: 0 }
    }

    pub fn new(addr: usize) -> Self {
        Addr { bits: addr }
    }

    // 转换成页表项
    pub fn to_pte_bits(&self) -> usize {
        (self.bits >> PAGE_BITS) << PTE_FLAGS_BITS
    }

    // 页内偏移
    pub fn page_offset(&self) -> usize {
        self.bits & PAGE_SIZE
    }
    // 是否页对齐
    pub fn aligned(&self) -> bool {
        self.page_offset() == 0
    }
    // 向下页对齐
    pub fn align_down(&self) -> Self {
        Self {
            bits: self.bits & !(PAGE_SIZE - 1),
        }
    }
    // 向上页对齐
    pub fn align_up(&self) -> Self {
        Self {
            bits: (self.bits + PAGE_SIZE - 1) & !(PAGE_SIZE - 1),
        }
    }
    pub fn add(&self, offset: usize) -> Self {
        Self {
            bits: self.bits + offset,
        }
    }

    // 获取虚拟地址的三级页表项偏移
    pub fn get_indexes(&self) -> [usize; 3] {
        let mut res = [0usize; 3];
        let mut va = self.bits >> PAGE_BITS;
        for i in 0..3 {
            res[i] = va & (PTE_NUM_PER_PAGE - 1);
            va >>= PTE_NUM_PER_PAGE_BITS;
        }
        res
    }
}

// 页面结构
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Page {
    pub addr: usize,
}

impl Debug for Page {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Page: {:#x}", self.addr))
    }
}

impl Page {
    pub fn empty() -> Self {
        Self { addr: 0 }
    }
    pub fn new(addr: usize) -> Self {
        Self {
            addr: addr & !(PAGE_SIZE - 1),
        }
    }

    // 清理页面
    pub fn clean_page(&self) {
        let dst = self.addr as *mut u8;
        unsafe {
            core::ptr::write_bytes(dst, 0, PAGE_SIZE);
        }
    }

    // 读取字节序列
    pub fn get_bytes(&self) -> &'static [u8] {
        let src = self.addr as *const u8;
        unsafe { core::slice::from_raw_parts(src, PAGE_SIZE) }
    }
    pub fn get_bytes_mut(&self) -> &'static mut [u8] {
        let src = self.addr as *mut u8;
        unsafe { core::slice::from_raw_parts_mut(src, PAGE_SIZE) }
    }

    // 读取页表项序列
    pub fn get_ptes(&self) -> &'static [PageTableEntry] {
        let src = self.addr as *const PageTableEntry;
        unsafe { core::slice::from_raw_parts(src, PTE_NUM_PER_PAGE) }
    }
    pub fn get_ptes_mut(&self) -> &'static mut [PageTableEntry] {
        let src = self.addr as *mut PageTableEntry;
        unsafe { core::slice::from_raw_parts_mut(src, PTE_NUM_PER_PAGE) }
    }
}

impl From<Addr> for Page {
    fn from(addr: Addr) -> Self {
        Self {
            addr: addr.bits & !(PAGE_SIZE - 1),
        }
    }
}
impl Into<Addr> for Page {
    fn into(self) -> Addr {
        Addr { bits: self.addr }
    }
}

impl<T> From<*mut T> for Page {
    fn from(ptr: *mut T) -> Self {
        Self { addr: ptr as usize }
    }
}
impl<T> Into<*mut T> for Page {
    fn into(self) -> *mut T {
        self.addr as *mut T
    }
}

use core::fmt::{self, Debug, Formatter};

use super::{
    page_table::PageTableEntry,
    param::{PAGE_BITS, PAGE_SIZE, PTE_NUM_PER_PAGE, PTE_NUM_PER_PAGE_BITS},
};

// 地址结构
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Addr {
    pub value: usize,
}

impl Debug for Addr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Addr: {:#x}", self.value))
    }
}

impl Addr {
    pub fn empty() -> Self {
        Addr { value: 0 }
    }

    pub fn new(addr: usize) -> Self {
        Addr { value: addr }
    }

    // 页内偏移
    pub fn page_offset(&self) -> usize {
        self.value & PAGE_SIZE
    }
    // 是否页对齐
    pub fn aligned(&self) -> bool {
        self.page_offset() == 0
    }
    // 向下页对齐
    pub fn align_down(&mut self) -> Self {
        self.value &= !(PAGE_SIZE - 1);
        *self
    }
    // 向上页对齐
    pub fn align_up(&mut self) -> Self {
        self.value = (self.value + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
        *self
    }

    // 获取虚拟地址的三级页表项偏移
    pub fn get_indexes(&self) -> [usize; 3] {
        let mut res = [0usize; 3];
        let mut va = self.value >> PAGE_BITS;
        for i in 0..3 {
            res[i] = va & (PTE_NUM_PER_PAGE - 1);
            va >>= PTE_NUM_PER_PAGE_BITS;
        }
        res
    }
}

// 页面结构
#[derive(Clone, Copy)]
pub struct Page {
    addr: usize,
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

    // 读取字节序列
    pub fn get_bytes(&self) -> &[u8] {
        let src = self.addr as *const u8;
        unsafe { core::slice::from_raw_parts(src, PAGE_SIZE) }
    }
    pub fn get_bytes_mut(&self) -> &mut [u8] {
        let src = self.addr as *mut u8;
        unsafe { core::slice::from_raw_parts_mut(src, PAGE_SIZE) }
    }

    // 读取页表项序列
    pub fn get_ptes(&self) -> &[PageTableEntry] {
        let src = self.addr as *const PageTableEntry;
        unsafe { core::slice::from_raw_parts(src, PAGE_SIZE) }
    }
    pub fn get_ptes_mut(&self) -> &mut [PageTableEntry] {
        let src = self.addr as *mut PageTableEntry;
        unsafe { core::slice::from_raw_parts_mut(src, PAGE_SIZE) }
    }
}

impl From<Addr> for Page {
    fn from(addr: Addr) -> Self {
        Self {
            addr: addr.value & !(PAGE_SIZE - 1),
        }
    }
}

impl Into<Addr> for Page {
    fn into(self) -> Addr {
        Addr { value: self.addr }
    }
}

use crate::mem::address::Page;
use crate::mem_layout::{
    MAX_PHYS_ADDR, MAX_VIRT_ADDR, MAX_VPN, PAGE_BITS, PAGE_SIZE, PTE_FLAGS_BITS,
};
use alloc::collections::BTreeMap;
use bitflags::*;
use core::fmt::{self, Debug, Formatter};
use rand::{rngs::SmallRng, Rng, SeedableRng};

use super::{
    address::Addr,
    page_allocator::{kalloc, PageTracker},
};

bitflags! {
    #[derive(PartialEq, Clone, Copy)]
    pub struct PTEFlags: u8 {
        const V = 1 << 0;
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
        const G = 1 << 5;
        const A = 1 << 6;
        const D = 1 << 7;
    }
}

// 页表项结构
#[repr(C)]
pub struct PageTableEntry {
    pub bits: usize,
}

impl Debug for PageTableEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("pte: {:#x}", self.bits))
    }
}

impl PageTableEntry {
    pub fn empty() -> Self {
        Self { bits: 0 }
    }
    pub fn new(pa: Addr, flags: PTEFlags) -> Self {
        Self {
            bits: pa.to_pte_bits() | flags.bits() as usize,
        }
    }

    // 获取成页表项指向的物理页面地址
    pub fn get_addr_bits(&self) -> usize {
        (self.bits >> PTE_FLAGS_BITS) << PAGE_BITS
    }
    // 获取页表项 flags
    pub fn flags(&self) -> PTEFlags {
        PTEFlags::from_bits(self.bits as u8).unwrap()
    }
    pub fn valid(&self) -> bool {
        (self.flags() & PTEFlags::V) != PTEFlags::empty()
    }
    pub fn readable(&self) -> bool {
        (self.flags() & PTEFlags::R) != PTEFlags::empty()
    }
    pub fn writable(&self) -> bool {
        (self.flags() & PTEFlags::W) != PTEFlags::empty()
    }
    pub fn executable(&self) -> bool {
        (self.flags() & PTEFlags::X) != PTEFlags::empty()
    }
    pub fn user(&self) -> bool {
        (self.flags() & PTEFlags::U) != PTEFlags::empty()
    }
}

pub struct PageTable {
    root: Addr,
    pages: BTreeMap<Addr, PageTracker>,
}

impl PageTable {
    pub fn new() -> Self {
        let page_tracker = kalloc().unwrap();
        let page = page_tracker.page();
        page.clean_page();
        let mut pages: BTreeMap<Addr, PageTracker> = BTreeMap::new();
        let root = Addr::new(page.addr);
        pages.insert(root, page_tracker);
        Self { root, pages }
    }

    pub fn push_page(&mut self, page_tracker: PageTracker) {
        self.pages
            .insert(Addr::new(page_tracker.page().addr), page_tracker);
    }

    // 释放一个页面
    pub fn remove_page(&mut self, addr: Addr) {
        self.pages.remove(&addr);
    }

    // 给定一个地址，获取其 0 级页表项
    pub fn walk(&self, va: Addr) -> Option<&mut PageTableEntry> {
        assert!(va.bits <= MAX_VIRT_ADDR);

        let indexs = va.get_indexes();
        let mut page: Page = self.root.into();
        let mut res: Option<&mut PageTableEntry> = None;
        for level in (0..3).rev() {
            let pte = &mut page.get_ptes_mut()[indexs[level]];
            if level == 0 {
                res = Some(pte);
                break;
            }
            if !pte.valid() {
                break;
            }
            page = Page::new(pte.get_addr_bits());
        }
        res
    }

    pub fn walk_alloc(&mut self, va: Addr) -> Option<&mut PageTableEntry> {
        assert!(va.bits <= MAX_VIRT_ADDR);

        let indexs = va.get_indexes();
        let mut page: Page = self.root.into();
        let mut res: Option<&mut PageTableEntry> = None;
        for level in (0..3).rev() {
            let pte = &mut page.get_ptes_mut()[indexs[level]];
            if level == 0 {
                res = Some(pte);
                break;
            }
            if !pte.valid() {
                if let Some(page_tracker) = kalloc() {
                    let page = page_tracker.page();
                    page.clean_page();
                    *pte = PageTableEntry::new(page.into(), PTEFlags::V);
                    self.pages.insert(page.into(), page_tracker);
                } else {
                    break; // 内存不足
                }
            }
            let pre = page;
            page = Page::new(pte.get_addr_bits());
        }
        res
    }

    // 给定一个虚拟地址，返回其物理地址，只能用于用户
    pub fn walk_addr(&self, va: Addr) -> Option<Addr> {
        assert!(va.bits <= MAX_VIRT_ADDR);

        if let Some(pte) = self.walk(va) {
            if !pte.valid() || !pte.user() {
                None
            } else {
                Some(Addr::new(pte.get_addr_bits()))
            }
        } else {
            None
        }
    }

    // 映射一个页面
    fn map(&mut self, va: Addr, pa: Addr, flags: PTEFlags) {
        assert!(va.bits <= MAX_VIRT_ADDR);
        assert!(pa.bits <= MAX_PHYS_ADDR);
        let pte = self.walk_alloc(va).unwrap();
        assert!(!pte.valid(), "{:?} has been mapped", *pte);
        *pte = PageTableEntry::new(pa, flags | PTEFlags::V);
    }

    pub fn contain(&self, key: Addr) -> bool {
        self.pages.contains_key(&key)
    }

    // 映射一段连续的页面
    pub fn map_range(&mut self, va: Addr, mut pa: Addr, len: usize, flags: PTEFlags) {
        let mut a = va.align_down();
        let last = va.add(len - 1).align_down();
        while (true) {
            self.map(a, pa, flags);
            if a == last {
                break;
            }
            a = a.add(PAGE_SIZE);
            pa = pa.add(PAGE_SIZE);
        }
    }

    // satp 寄存器存放的2级页表地址
    pub fn make_satp(&self) -> usize {
        (8usize << 60) | (self.root.bits >> PAGE_BITS)
    }

    // 打印页表
    pub fn print_page_table(&self) {
        let mut page: Page = self.root.into();
        println!("pagetable: {:?}", page);
        for (i, pte) in page.get_ptes().into_iter().enumerate() {
            if !pte.valid() {
                continue;
            }
            println!("  ..{:?}, pa: {:?}", *pte, Addr::new(pte.get_addr_bits()));
            page = Page::new(pte.get_addr_bits());
            for (i, pte) in page.get_ptes().into_iter().enumerate() {
                if !pte.valid() {
                    continue;
                }
                println!("    ..{:?}, pa: {:?}", *pte, Addr::new(pte.get_addr_bits()));
                page = Page::new(pte.get_addr_bits());
                for (i, pte) in page.get_ptes().into_iter().enumerate() {
                    if !pte.valid() {
                        continue;
                    }
                    println!(
                        "      ..{:?}, pa: {:?}",
                        *pte,
                        Addr::new(pte.get_addr_bits())
                    );
                }
            }
        }
        println!("print page table success");
    }

    // 随机生成内核代码段的虚拟地址，与 walk_addr() 的结果是否一致
    pub fn walk_test(&self) {
        let mut va = Addr::empty();
        let mut pa = Addr::empty();
        let mut rng = SmallRng::seed_from_u64(0);
        for _ in 0..10 {
            va = Addr::new(rng.gen_range(0x8020_0000..0x8080_0000));
            pa = self.walk_addr(va).unwrap();
            println!("walk va: {:?} -> pa: {:?}", va, pa);
        }
    }
}

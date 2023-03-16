use alloc::vec::Vec;
use core::fmt::{self, Debug, Formatter};
use core::ptr::null_mut;
use lazy_static::*;

use crate::sync::UPSafeCell;

use super::{
    address::{Addr, Page},
    param::{PAGE_SIZE, PHYS_TOP},
};

// 页面分配器必须实现这个特征
trait PageAlloc {
    fn empty() -> Self;
    fn init(&mut self, start: Addr, end: Addr);
    fn alloc(&mut self) -> Option<Page>;
    fn dealloc(&mut self, page: Page);
}

#[repr(C)]
struct FreeListNode {
    next: *mut FreeListNode,
}

// 空闲页面链表
struct FreeList {
    head: *mut FreeListNode,
    rear: *mut FreeListNode,
}

impl FreeList {
    pub fn empty() -> Self {
        Self {
            head: null_mut(),
            rear: null_mut(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.head == null_mut()
    }
    pub fn push_back(&mut self, node: *mut FreeListNode) {
        if node != null_mut() {
            unsafe {
                (*node).next = null_mut();
            }
            if self.is_empty() {
                self.head = node;
                self.rear = node;
            } else {
                unsafe {
                    (*self.rear).next = node;
                }
                self.rear = node;
            }
        }
    }
    pub fn pop_front(&mut self) -> Option<*mut FreeListNode> {
        let mut res = self.head;
        if self.head != null_mut() {
            unsafe {
                self.head = (*res).next;
            }
            if self.head == null_mut() {
                self.rear = null_mut()
            }
            Some(res)
        } else {
            None
        }
    }
}

// 链式物理页分配器
pub struct PageAllocator {
    next: Addr,
    end: Addr,
    free_list: FreeList,
}

impl PageAlloc for PageAllocator {
    fn empty() -> Self {
        Self {
            next: Addr::empty(),
            end: Addr::empty(),
            free_list: FreeList::empty(),
        }
    }
    fn init(&mut self, start: Addr, end: Addr) {
        self.next = start;
        self.end = end;
    }
    fn alloc(&mut self) -> Option<Page> {
        if self.next > self.end {
            if let Some(addr) = self.free_list.pop_front() {
                Some(addr.into())
            } else {
                None
            }
        } else {
            let res = self.next;
            self.next.bits += PAGE_SIZE;
            Some(res.into())
        }
    }
    fn dealloc(&mut self, page: Page) {
        self.free_list.push_back(page.into())
    }
}

lazy_static! {
    pub static ref PAGE_ALLOCATOR: UPSafeCell<PageAllocator> =
        unsafe { UPSafeCell::new(PageAllocator::empty()) };
}

pub struct PageTracker {
    page: Page,
}

impl Debug for PageTracker {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("PageTracker: page = {:#x}", self.page.addr))
    }
}

impl Drop for PageTracker {
    fn drop(&mut self) {
        kfree(self.page);
    }
}

impl PageTracker {
    pub fn new(page: Page) -> Self {
        Self { page }
    }
    pub fn page(&self) -> Page {
        self.page
    }
}

pub fn kinit() {
    extern "C" {
        fn ekernel();
    }
    PAGE_ALLOCATOR.get_mut().init(
        Addr::new(ekernel as usize).align_up(),
        Addr::new(PHYS_TOP).align_down(),
    );
}

pub fn kalloc() -> Option<PageTracker> {
    PAGE_ALLOCATOR.get_mut().alloc().map(PageTracker::new)
}

fn kfree(page: Page) {
    PAGE_ALLOCATOR.get_mut().dealloc(page);
}

pub fn page_allocator_test() {
    let mut v: Vec<PageTracker> = Vec::new();
    for i in 0..5 {
        let page = kalloc().unwrap();
        println!("{:?}", page);
        v.push(page);
    }
    v.clear();
    for i in 0..5 {
        let page = kalloc().unwrap();
        println!("{:?}", page);
        v.push(page);
    }
    v.clear();
    for i in 0..5 {
        let page = kalloc().unwrap();
        println!("{:?}", page);
        v.push(page);
    }
    drop(v);
    println!("page allocator test passed!");
}

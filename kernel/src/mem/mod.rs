use crate::mem_layout::{PAGE_SIZE, TRAMPOLINE};

use self::address::Addr;

mod address;
mod kernel_heap;
mod kernel_space;
mod page_allocator;
mod page_table;
pub mod user_space;

pub fn kernel_stack_i(id: usize) -> Addr {
    Addr::new(TRAMPOLINE - 2 * (id + 1) * PAGE_SIZE)
}

pub fn init() {
    kernel_heap::init_heap();
    // kernel_heap::kernel_heap_test();

    page_allocator::kinit();
    page_allocator::page_allocator_test();
    kernel_space::kvminit();

    user_space::userspace_test();

    println!("memory init success!");
}

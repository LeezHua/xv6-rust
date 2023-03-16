mod address;
mod kernel_heap;
mod kernel_space;
mod page_allocator;
mod page_table;
pub mod param;

pub fn init() {
    kernel_heap::init_heap();
    kernel_heap::kernel_heap_test();

    page_allocator::kinit();
    page_allocator::page_allocator_test();
    kernel_space::kvminit();

    println!("memory init success!");
}

pub mod address;
pub mod kernel_heap;
pub mod page_table;
pub mod param;

pub fn init() {
    kernel_heap::init_heap();
    kernel_heap::kernel_heap_test();

    println!("memory init success!");
}

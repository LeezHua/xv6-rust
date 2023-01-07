#![no_std]
#![feature(default_alloc_error_handler)]
#![feature(c_size_t)]
use libc;

extern "C" {
	fn cpuid() -> libc::c_int;
	fn consoleinit();
	fn printfinit();
	fn kinit();
	fn kvminit();
	fn procinit();
	fn trapinit();
	fn trapinithart();
	fn plicinit();
	fn plicinithart();
	fn binit();
	fn iinit();
	fn fileinit();
	fn virtio_disk_init();
	fn userinit();
	fn kvminithart();
	fn scheduler();
}

static mut STARTED: libc::c_int = 0;

fn main() {
	unsafe {
		if cpuid() == 0 {
			consoleinit();
			printfinit();
			println!("\n");
			println!("xv6 kernel is booting\n");
			println!("\n");
			kinit();
			kvminit();
			kvminithart();
			procinit();
			trapinit();
			trapinithart();
			plicinit();
			plicinithart();
			binit();
			iinit();
			fileinit();
			virtio_disk_init();
			userinit();

			STARTED = 1;
		} else {
			println!("hart {} starting\n", cpuid());
			kvminithart();
			trapinithart();
			plicinithart();
		}
		scheduler();
	}
}
/* memory relative parameter */
pub const KERNEL_STACK_SIZE: usize = 0x2000;
pub const USER_STACK_SIZE: usize = 0x2000;
pub const KERNEL_HEAP_SIZE: usize = 0x20_0000; // kernel heap allocator size
pub const PAGE_SIZE: usize = 4096; // bytes per page
pub const PAGE_BITS: usize = 12; // bits of offset within a page
pub const PTE_FLAGS_BITS: usize = 10;
pub const PTE_NUM_PER_PAGE: usize = 1 << PTE_NUM_PER_PAGE_BITS;
pub const PTE_NUM_PER_PAGE_BITS: usize = 9;

// physical address & virtual address
pub const PA_WIDTH_SV39: usize = 56;
pub const VA_WIDTH_SV39: usize = 39;
pub const MAX_PHYS_ADDR: usize = (1 << PA_WIDTH_SV39) - 1;
pub const MAX_VIRT_ADDR: usize = (1 << VA_WIDTH_SV39) - 1;
pub const MAX_PHYS_SIZE: usize = 1 << (PA_WIDTH_SV39 - 1);
pub const MAX_VIRT_SIZE: usize = 1 << (VA_WIDTH_SV39 - 1);

// physical page number & virtual page number
pub const PPN_WIDTH_SV39: usize = PA_WIDTH_SV39 - PAGE_BITS;
pub const VPN_WIDTH_SV39: usize = VA_WIDTH_SV39 - PAGE_BITS;
pub const MAX_PPN: usize = (1 << PPN_WIDTH_SV39) - 1;
pub const MAX_VPN: usize = (1 << VPN_WIDTH_SV39) - 1;

/* memory layout */

pub const KERNEL_BASE: usize = 0x8020_0000;
pub const PHYS_TOP: usize = 0x8800_0000;
pub const TRAMPOLINE: usize = MAX_VIRT_SIZE - PAGE_SIZE;
pub const TRAP_FRAME: usize = TRAMPOLINE - PAGE_SIZE;

pub const MAX_BUF_SIZE: usize = 1024;

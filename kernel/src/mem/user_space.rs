use core::usize::MIN;

use crate::{
    mem::{address::Addr, page_table::PTEFlags},
    mem_layout::{PAGE_SIZE, TRAMPOLINE, TRAP_FRAME, USER_STACK_SIZE},
    sync::UPSafeCell,
};
use alloc::collections::BTreeMap;
use lazy_static::*;

use super::{
    page_allocator::{kalloc, PageTracker},
    page_table::PageTable,
};

pub struct UserSpace {
    page_table: PageTable,
    data_pages: BTreeMap<Addr, PageTracker>,
    size: usize,
}

impl UserSpace {
    pub fn new() -> Self {
        Self {
            page_table: PageTable::new(),
            data_pages: BTreeMap::new(),
            size: 0,
        }
    }

    fn alloc(&mut self, mut start: Addr, end: Addr, perm: PTEFlags) {
        while (start < end) {
            let page_tracker = kalloc().unwrap();
            let pa: Addr = page_tracker.page().into();
            self.data_pages.insert(pa, page_tracker);
            self.size += PAGE_SIZE;
            self.page_table.map(start, pa, perm | PTEFlags::U);

            start = start.add(PAGE_SIZE);
        }
    }

    fn load_segment(&mut self, va: Addr, data: &[u8]) {
        let data_len = data.len();
        let mut size = 0usize;
        while (size < data_len) {
            let pa = self.page_table.walk_addr(va.add(size)).unwrap();
            let len = core::cmp::min(PAGE_SIZE, data_len - size);
            let dst = unsafe { core::slice::from_raw_parts_mut(pa.bits as *mut u8, len) };
            dst.copy_from_slice(&data[size..(size + len)]);
            size += len;
        }
    }

    // 映射 trampoline 和 trampframe
    fn init_pagetable(&mut self) -> Addr {
        extern "C" {
            fn trampoline();
        }

        // 映射 trampoline
        self.page_table.map(
            Addr::new(TRAMPOLINE),
            Addr::new(trampoline as usize),
            PTEFlags::R | PTEFlags::X,
        );

        // 为 trapframe 分配内存
        let page_tracker = kalloc().unwrap();
        let pa: Addr = page_tracker.page().into();
        self.data_pages.insert(pa, page_tracker);

        // 映射 trapframe
        self.page_table
            .map(Addr::new(TRAP_FRAME), pa, PTEFlags::R | PTEFlags::W);

        // 返回 trapframe 的物理地址
        pa
    }

    fn init_stack(&mut self, va: Addr) -> Addr {
        let mut a = 0usize;
        while (a < USER_STACK_SIZE) {
            let page_tracker = kalloc().unwrap();
            let pa: Addr = page_tracker.page().into();
            self.data_pages.insert(pa, page_tracker);

            self.page_table
                .map(va.add(a), pa, PTEFlags::R | PTEFlags::W | PTEFlags::U);
            a += PAGE_SIZE;
        }
        self.page_table.walk_addr(va).unwrap()
    }

    pub fn init_from_elf(&mut self, elf_data: &[u8]) -> (Addr, Addr) {
        let trap_frame = self.init_pagetable();

        let elf = xmas_elf::ElfFile::new(elf_data).unwrap();
        let elf_header = elf.header;
        let magic = elf_header.pt1.magic;
        assert_eq!(magic, [0x7f, 0x45, 0x4c, 0x46], "invalid elf!");

        let ph_count = elf_header.pt2.ph_count();

        let mut prog_end = Addr::new(0);
        for i in 0..ph_count {
            let ph = elf.program_header(i).unwrap();
            if ph.get_type().unwrap() == xmas_elf::program::Type::Load {
                let start = Addr::new(ph.virtual_addr() as usize);
                let end = Addr::new((ph.virtual_addr() + ph.mem_size()) as usize);

                let flags = ph.flags();
                let mut perm = PTEFlags::empty();
                if flags.is_read() {
                    perm |= PTEFlags::R;
                }
                if flags.is_write() {
                    perm |= PTEFlags::W;
                }
                if flags.is_execute() {
                    perm |= PTEFlags::X;
                }
                self.alloc(start, end, perm);
                self.load_segment(
                    start,
                    &elf.input[ph.offset() as usize..(ph.offset() + ph.file_size()) as usize],
                );
                prog_end = end;
            }
        }

        let stack_bottom = self.init_stack(prog_end.add(PAGE_SIZE));

        (stack_bottom, trap_frame)
    }
    pub fn print_user_pagetable(&self) {
        self.page_table.print_page_table();
    }
}

pub fn userspace_test() {
    extern "C" {
        fn _app_num();
    }

    let app_num = unsafe { *(_app_num as *const usize) };
    let app_start =
        unsafe { core::slice::from_raw_parts((_app_num as usize + 8) as *const usize, app_num) };
    let app0 = unsafe {
        core::slice::from_raw_parts(app_start[0] as *const u8, app_start[1] - app_start[0])
    };

    let mut user = UserSpace::new();
    user.init_from_elf(app0);
    user.print_user_pagetable();
    println!("user space test success!");
}

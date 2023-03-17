use core::usize::MIN;

use crate::{
    mem::{address::Addr, page_table::PTEFlags},
    sync::UPSafeCell,
};
use lazy_static::*;

use super::{page_allocator::kalloc, page_table::PageTable, param::PAGE_SIZE};

pub struct UserSpace {
    page_table: PageTable,
    size: usize,
}

impl UserSpace {
    pub fn new() -> Self {
        Self {
            page_table: PageTable::new(),
            size: 0,
        }
    }

    fn alloc(&mut self, mut start: Addr, end: Addr, perm: PTEFlags) {
        while (start < end) {
            let page_tracker = kalloc().unwrap();
            let pa: Addr = page_tracker.page().into();
            self.page_table.push_page(page_tracker);
            self.size += PAGE_SIZE;
            self.page_table
                .map_range(start, pa, PAGE_SIZE, perm | PTEFlags::U);

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
    fn init(&self) {
        // todo!()
    }

    pub fn elf_init(&mut self, elf_data: &[u8]) {
        self.init();

        let elf = xmas_elf::ElfFile::new(elf_data).unwrap();
        let elf_header = elf.header;
        let magic = elf_header.pt1.magic;
        assert_eq!(magic, [0x7f, 0x45, 0x4c, 0x46], "invalid elf!");

        let ph_count = elf_header.pt2.ph_count();

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
            }
        }
    }
    pub fn print_user_page_table(&self) {
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
    user.elf_init(app0);
    user.print_user_page_table();
    println!("user space test success!");
}

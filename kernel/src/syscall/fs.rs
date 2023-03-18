//! File and filesystem-related syscalls

use crate::{
    mem::{address::Addr, copy_from_user},
    mem_layout::PAGE_BITS,
    task::current_user_satp,
};

const FD_SDTIN: usize = 0;
const FD_STDOUT: usize = 1;
const FD_STDERR: usize = 2;

/// write buf of length `len`  to a file with `fd`
pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    let buf_usize = buf as usize;
    match fd {
        FD_STDOUT => {
            let buffer = copy_from_user(
                Addr::new(current_user_satp() << PAGE_BITS),
                Addr::new(buf_usize),
                len,
            );
            let string = unsafe { core::str::from_utf8(buffer).unwrap() };
            print!("{}", string);
            len as isize
        }
        _ => {
            panic!("Unsupported fd in sys_write!");
        }
    }
}

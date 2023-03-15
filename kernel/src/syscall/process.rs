//! App management syscalls

use crate::task::run_next_task_kill;

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> isize {
    println!("[kernel] Task exited with code {}", exit_code);
    run_next_task_kill();
    0
}

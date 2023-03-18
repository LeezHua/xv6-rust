use core::arch::global_asm;

use alloc::vec::Vec;
use lazy_static::lazy_static;

use crate::{
    board::{QEMUExit, QEMU_EXIT_HANDLE},
    sync::UPSafeCell,
    trap::TrapContext,
};

use self::{
    context::TaskContext,
    loader::{get_app_data, get_app_num},
    param::MAX_APP_NUM,
    task::{TaskControlBlock, TaskStatus},
};

global_asm!(include_str!("switch.S"));

mod context;
mod loader;
pub mod param;
#[allow(clippy::module_inception)]
pub mod task;

extern "C" {
    fn switch(old_cx: *mut TaskContext, new_cx: *const TaskContext);
}

pub struct TaskManagerInner {
    tasks: Vec<TaskControlBlock>,
    current: usize,
}

pub struct TaskManager {
    task_num: usize,
    inner: UPSafeCell<TaskManagerInner>,
}

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = TaskManager {
        task_num: get_app_num(),
        inner: UPSafeCell::new({
            let tasks: Vec<TaskControlBlock> =
                (0..MAX_APP_NUM).map(|_| TaskControlBlock::new()).collect();
            TaskManagerInner { tasks, current: 0 }
        })
    };
}

impl TaskManager {
    fn load_tasks(&self) {
        let mut inner = self.inner.get_mut();
        let tasks = &mut inner.tasks;

        for id in 0..self.task_num {
            tasks[id].init_from_elf(get_app_data(id), id);
        }
    }

    fn run_first_task(&self) -> ! {
        let mut inner = self.inner.get_mut();
        let current = inner.current;
        inner.tasks[current].status = TaskStatus::Running;
        let mut zero = TaskContext::new();
        let first_cx = &mut inner.tasks[current].context as *const TaskContext;
        drop(inner);

        println!("[kernle] task{} running.", current);
        unsafe {
            switch(&mut zero as *mut TaskContext, first_cx);
        }
        panic!("Unreachaable in run_first_task!");
    }

    fn mark_current_runnable(&self) {
        let mut inner = self.inner.get_mut();
        let current = inner.current;
        inner.tasks[current].status = TaskStatus::Runable;
    }

    fn mark_current_zombie(&self) {
        let mut inner = self.inner.get_mut();
        let current = inner.current;
        inner.tasks[current].status = TaskStatus::Zombie;
    }

    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.get_mut();
        let current = inner.current;
        ((current + 1)..(current + 1 + self.task_num))
            .map(|x| x % self.task_num)
            .find(|&id| inner.tasks[id].status == TaskStatus::Runable)
    }

    fn run_next_task(&self) {
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.get_mut();
            let current = inner.current;
            inner.tasks[next].status = TaskStatus::Running;
            inner.current = next;
            let mut old_cx = &mut inner.tasks[current].context as *mut TaskContext;
            let new_cx = &inner.tasks[next].context as *const TaskContext;

            drop(inner);
            println!("[kernel] task{} running", next);
            unsafe {
                switch(old_cx, new_cx);
            }
        } else {
            println!("[kernel] All tasks completed!");
            QEMU_EXIT_HANDLE.exit_success();
        }
    }

    fn current_user_satp(&self) -> usize {
        let inner = self.inner.get_mut();
        let current = inner.current;
        inner.tasks[current].user_satp()
    }

    fn current_user_epc(&self) -> usize {
        let inner = self.inner.get_mut();
        let current = inner.current;
        inner.tasks[current].user_epc()
    }

    fn current_pagetable(&self) {
        let inner = self.inner.get_mut();
        let current = inner.current;
        inner.tasks[current].space.print_user_pagetable();
    }

    fn current_user_trapcontex(&self) -> &'static mut TrapContext {
        let inner = self.inner.get_mut();
        let current = inner.current;
        inner.tasks[current].trap_context()
    }
}

pub fn load_tasks() {
    TASK_MANAGER.load_tasks();
}

pub fn run_first_task() {
    TASK_MANAGER.run_first_task();
}

pub fn run_next_task_kill() {
    TASK_MANAGER.mark_current_zombie();
    TASK_MANAGER.run_next_task();
}

pub fn run_next_task_suspend() {
    TASK_MANAGER.mark_current_runnable();
    TASK_MANAGER.run_next_task();
}

pub fn current_user_satp() -> usize {
    TASK_MANAGER.current_user_satp()
}

pub fn current_user_epc() -> usize {
    TASK_MANAGER.current_user_epc()
}

pub fn current_pagetable() {
    TASK_MANAGER.current_pagetable();
}

pub fn current_user_trapcontext() -> &'static mut TrapContext {
    TASK_MANAGER.current_user_trapcontex()
}

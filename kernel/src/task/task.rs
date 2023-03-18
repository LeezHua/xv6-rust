use crate::{
    mem::{
        address::{Addr, Page},
        kernel_sp_i,
        user_space::UserSpace,
    },
    trap::{user_trap_handler, TrapContext},
};

use super::{param::APP_BASE_ADDRESS, TaskContext};

pub struct TaskControlBlock {
    pub status: TaskStatus,
    pub context: TaskContext,
    pub space: UserSpace,
    pub trapframe: Addr,
}

impl TaskControlBlock {
    pub fn new() -> Self {
        Self {
            status: TaskStatus::Unused,
            context: TaskContext::new(),
            space: UserSpace::empty(),
            trapframe: Addr::empty(),
        }
    }

    pub fn clear(&mut self) {}

    pub fn init_from_elf(&mut self, elf_data: &[u8], id: usize) {
        let (sp, trapframe) = self.space.init_from_elf(elf_data); // 为用户程序分配内存, 并开启页面映射
        self.trapframe = trapframe; // 设置 trapframe 指针

        // 初始化用户程序的 trapcontext, 用以第一次被执行
        let tf_ptr = trapframe.get_value_mut::<TrapContext>();
        *tf_ptr = TrapContext::app_init_context(
            APP_BASE_ADDRESS,
            sp.bits,
            self.space.make_satp(),
            kernel_sp_i(id),
            user_trap_handler as usize,
        );

        // 初始化用户程序的 taskcontext, 用以内核线程之间的切换
        self.context.init(kernel_sp_i(id));

        // 设置程序状态为 runable
        self.status = TaskStatus::Runable;

        println!("[kernel] init task{} success", id);
    }

    pub fn user_satp(&self) -> usize {
        self.space.make_satp()
    }

    pub fn user_epc(&self) -> usize {
        let tf_ptr = self.trapframe.get_value::<TrapContext>();
        tf_ptr.epc
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum TaskStatus {
    Unused,
    Used,
    Sleeping,
    Runable,
    Running,
    Zombie,
}

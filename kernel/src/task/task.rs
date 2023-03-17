use crate::mem::user_space::UserSpace;

use super::TaskContext;

pub struct TaskControlBlock {
    pub status: TaskStatus,
    pub context: TaskContext,
    pub space: UserSpace,
}

impl TaskControlBlock {
    pub fn new() -> Self {
        Self {
            status: TaskStatus::Unused,
            context: TaskContext::new(),
            space: UserSpace::new(),
        }
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

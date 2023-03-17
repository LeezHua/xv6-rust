use riscv::register::sstatus::Sstatus;

#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32],
    kernel_satp: usize, // kernel page table
    kernel_sp: usize,   // top of process's kernel stack
    kernel_trap: usize, // user_trap_hanbler()
    pub epc: usize,     // saved user program counter
}

impl TrapContext {
    pub fn new(kernel_satp: usize, kernel_sp: usize, kernel_trap: usize) -> Self {
        Self {
            x: [0; 32],
            kernel_satp,
            kernel_sp,
            kernel_trap,
            epc: 0,
        }
    }

    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }
}

use bitflags::bitflags;

use crate::cpu::*
use crate::task::*


bitflags! {
    pub struct Status: u8 {
        const TASK = 0x00;
        const NON_TASK = 0x01;
        const CPU_LOCK = 0x02;
        const DISABLE_DSP = 0x04;
    }
}

pub struct SystemControlBlock {
    status: ContextStatus,
    dispatch_pending: bool,
}

static mut SYSCB: SystemControlBlock = SystemControlBlock {
    status: ContextStatus::TASK,
    dispatch_pending: false,
};


pub fn enter_system_call() {
    cpu_lock();
}

pub fn leave_system_call() {
    unsafe {
        if !SYSCB.status.contains(ContextStatus::DISABLE_DSP) {
            task_switch();
        }

        if !SYSCB.status.contains(ContextStatus::CPU_LOCK) {
            cpu_unlock();
        }
    }
}


pub struct SystemCall {
}

impl SystemCall {
    pub fn new() -> Self {
        enter_system_call();
        Self { }
    }
}

impl Drop for SystemCall {
    fn drop(&mut self) {
        leave_system_call();
    }
}




pub fn set_non_task_state() {
    unsafe {
        SYSCB.status = SYSCB.status | ContextStatus::NON_TASK;
    }
}

pub fn is_dispatch_pending_state() -> bool {
    unsafe { SYSCB.dispatch_pending }
}

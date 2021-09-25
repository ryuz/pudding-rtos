
use crate::cpu::*;
use crate::task::*;

/*
use bitflags::bitflags;
bitflags! {
    pub struct Status: u8 {
        const TASK = 0x00;
        const NON_TASK = 0x01;
        const CPU_LOCK = 0x02;
        const DISABLE_DSP = 0x04;
    }
}
*/

struct SystemControlBlock {
    cpu_lock: bool,
    dispatch_disable: bool,
    dispatch_pending: bool,
}


static mut SYSCB: SystemControlBlock = SystemControlBlock {
    cpu_lock: false,
    dispatch_disable: false,
    dispatch_pending: false,
};


pub (crate) unsafe fn _kernel_get_cpu_lock() -> bool {
    SYSCB.cpu_lock
}

pub (crate) unsafe fn _kernel_set_cpu_lock(cpu_lock: bool) {
    SYSCB.cpu_lock = cpu_lock;
}

pub (crate) unsafe fn _kernel_get_dispatch_disable() -> bool {
    SYSCB.dispatch_disable
}

pub (crate) unsafe fn _kernel_set_dispatch_disable(dispatch_disable: bool) {
    SYSCB.dispatch_disable = dispatch_disable;
}


pub (crate) unsafe fn _kernel_get_dispatch_pending() -> bool {
    SYSCB.dispatch_pending
}

pub (crate) unsafe fn _kernel_set_dispatch_pending(dispatch_pending: bool) {
    SYSCB.dispatch_pending = dispatch_pending;
}



pub (crate) fn enter_system_call() {
    unsafe {
        crate::cpu::cpu_lock();
    }
}

pub (crate) fn leave_system_call() {
     unsafe {
        if _kernel_get_dispatch_pending() && !_kernel_get_dispatch_disable() {
            _kernel_set_dispatch_pending(false);
            task_switch();
        }
        
        if !_kernel_get_cpu_lock() {
            cpu_unlock();
        }
    }
}


pub (crate) struct SystemCall {
}

impl SystemCall {
    pub (crate) fn new() -> Self {
        enter_system_call();
        Self { }
    }
}

impl Drop for SystemCall {
    fn drop(&mut self) {
        leave_system_call();
    }
}


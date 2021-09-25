
use crate::cpu::*;
use crate::task::*;


struct SystemControlBlock {
    interrupt: bool,
    cpu_lock: bool,
    dispatch_disable: bool,
    dispatch_reserve: bool,
}


static mut SYSCB: SystemControlBlock = SystemControlBlock {
    interrupt: false,
    cpu_lock: false,
    dispatch_disable: false,
    dispatch_reserve: false,
};

pub (crate) unsafe fn test_interrupt_flag() -> bool {
    SYSCB.interrupt
}

pub (crate) unsafe fn set_interrupt_flag() {
    SYSCB.interrupt = true;
}

pub (crate) unsafe fn clear_interrupt_flag() {
    SYSCB.interrupt = false;
}


pub (crate) unsafe fn test_cpu_lock_flag() -> bool {
    SYSCB.cpu_lock
}

pub (crate) unsafe fn set_cpu_lock_flag() {
    SYSCB.cpu_lock = true;
}

pub (crate) unsafe fn clear_cpu_lock_flag() {
    SYSCB.cpu_lock = false;
}


pub (crate) unsafe fn test_dispatch_disable_flag() -> bool {
    SYSCB.dispatch_disable
}

pub (crate) unsafe fn set_dispatch_disable_flag() {
    SYSCB.dispatch_disable = true;
}

pub (crate) unsafe fn clear_dispatch_disable_flag() {
    SYSCB.dispatch_disable = false;
}



pub (crate) unsafe fn test_dispatch_reserve_flag() -> bool {
    SYSCB.dispatch_reserve
}

pub (crate) unsafe fn set_dispatch_reserve_flag() {
    SYSCB.dispatch_reserve = true;
}

pub (crate) unsafe fn clear_dispatch_reserve_flag() {
    SYSCB.dispatch_reserve = false;
}


pub (crate) fn enter_system_call() {
    unsafe {
        cpu_lock();
    }
}

pub (crate) unsafe fn leave_system_call() {
    if test_dispatch_reserve_flag() && !test_interrupt_flag() && !test_dispatch_disable_flag() && !test_cpu_lock_flag() {
        clear_dispatch_reserve_flag();
        task_switch();
    }
    
    if !test_cpu_lock_flag() {
        cpu_unlock();
    }
}


pub (crate) struct SystemCall {
}

impl SystemCall {
    pub (crate) unsafe fn new() -> Self {
        enter_system_call();
        Self { }
    }
}

impl Drop for SystemCall {
    fn drop(&mut self) {
        unsafe {
            leave_system_call();
        }
    }
}


pub fn lock_cpu() {
    unsafe {
        let _sc = SystemCall::new();
        set_cpu_lock_flag();
    }
}

pub fn unlock_cpu() {
    unsafe {
        let _sc = SystemCall::new();
        clear_cpu_lock_flag();
    }
}

pub fn is_cpu_locked() -> bool {
    unsafe {
        let _sc = SystemCall::new();
        test_cpu_lock_flag()
    }
}


pub fn disable_dispatch() {
    unsafe {
        let _sc = SystemCall::new();
        set_dispatch_disable_flag();
    }
}

pub fn enable_dispatch() {
    unsafe {
        let _sc = SystemCall::new();
        clear_dispatch_disable_flag();
    }
}

pub fn is_dispatch_disabled() -> bool {
    unsafe {
        let _sc = SystemCall::new();
        test_dispatch_disable_flag()
    }
}

pub fn is_dispatch_pending_state() -> bool {
    unsafe {
        let _sc = SystemCall::new();
        test_cpu_lock_flag() || test_dispatch_disable_flag()
    }
}

pub fn idle_loop() -> ! {
    loop {
        unsafe { cpu_halt(); }
    }
}

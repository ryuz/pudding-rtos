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

pub(crate) fn test_interrupt_flag() -> bool {
    unsafe { SYSCB.interrupt }
}

pub(crate) fn set_interrupt_flag() {
    unsafe {
        SYSCB.interrupt = true;
    }
}

pub(crate) fn clear_interrupt_flag() {
    unsafe {
        SYSCB.interrupt = false;
    }
}

pub(crate) fn test_cpu_lock_flag() -> bool {
    unsafe { SYSCB.cpu_lock }
}

pub(crate) fn set_cpu_lock_flag() {
    unsafe {
        SYSCB.cpu_lock = true;
    }
}

pub(crate) fn clear_cpu_lock_flag() {
    unsafe {
        SYSCB.cpu_lock = false;
    }
}

pub(crate) fn test_dispatch_disable_flag() -> bool {
    unsafe { SYSCB.dispatch_disable }
}

pub(crate) fn set_dispatch_disable_flag() {
    unsafe {
        SYSCB.dispatch_disable = true;
    }
}

pub(crate) fn clear_dispatch_disable_flag() {
    unsafe {
        SYSCB.dispatch_disable = false;
    }
}

pub(crate) fn test_dispatch_reserve_flag() -> bool {
    unsafe { SYSCB.dispatch_reserve }
}

pub(crate) fn set_dispatch_reserve_flag() {
    unsafe {
        SYSCB.dispatch_reserve = true;
    }
}

pub(crate) fn clear_dispatch_reserve_flag() {
    unsafe {
        SYSCB.dispatch_reserve = false;
    }
}

// System Call

pub(crate) unsafe fn enter_system_call() {
    cpu_lock();
    // (注) もしマルチプロセッサやる場合はここにメモリバリアも入れる
}

pub(crate) unsafe fn leave_system_call() {
    if test_dispatch_reserve_flag()
        && !test_interrupt_flag()
        && !test_dispatch_disable_flag()
        && !test_cpu_lock_flag()
    // (注) もしマルチプロセッサやる場合はここにメモリバリアも入れる
    {
        clear_dispatch_reserve_flag();
        task_switch();
    }

    if !test_cpu_lock_flag() {
        cpu_unlock();
    }
}

pub(crate) struct SystemCall {}

impl SystemCall {
    pub(crate) fn new() -> Self {
        unsafe {
            enter_system_call();
            Self {}
        }
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
    let _sc = SystemCall::new();
    set_cpu_lock_flag();
}

pub fn unlock_cpu() {
    let _sc = SystemCall::new();
    clear_cpu_lock_flag();
}

pub fn is_cpu_locked() -> bool {
    let _sc = SystemCall::new();
    test_cpu_lock_flag()
}

pub fn disable_dispatch() {
    let _sc = SystemCall::new();
    set_dispatch_disable_flag();
}

pub fn enable_dispatch() {
    let _sc = SystemCall::new();
    clear_dispatch_disable_flag();
}

pub fn is_dispatch_disabled() -> bool {
    let _sc = SystemCall::new();
    test_dispatch_disable_flag()
}

pub fn is_dispatch_pending_state() -> bool {
    let _sc = SystemCall::new();
    test_cpu_lock_flag() || test_dispatch_disable_flag()
}

pub fn is_interrupt_state() -> bool {
    test_interrupt_flag()
}

pub fn idle_loop() -> ! {
    loop {
        unsafe {
            cpu_halt();
        }
    }
}

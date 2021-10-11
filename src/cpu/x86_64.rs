

#[repr(C)]
pub struct Context {
    pub sp: usize,
}

impl Context {
    pub const fn new() -> Self {
        Context{ sp: 0 }
    }
}

/*
#[macro_export]
macro_rules! context_default {
    () => {
        Context { sp: 0 }
    };
}
*/

pub (crate) unsafe fn cpu_initialize() {}

pub (crate) unsafe fn interrupt_initialize(_stack: &mut [isize]) {}

pub (crate) unsafe fn cpu_lock() {}

pub (crate) unsafe fn cpu_unlock() {}

pub (crate) unsafe fn cpu_halt() {}


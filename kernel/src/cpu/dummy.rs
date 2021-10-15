

pub unsafe fn cpu_initialize() {}
pub unsafe fn cpu_lock() {}
pub unsafe fn cpu_unlock() {}
pub unsafe fn cpu_halt() {}

pub unsafe fn interrupt_initialize(_stack: &mut [u8]) {}

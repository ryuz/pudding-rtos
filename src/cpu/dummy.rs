

pub (crate) unsafe fn cpu_initialize() {}
pub (crate) unsafe fn cpu_lock() {}
pub (crate) unsafe fn cpu_unlock() {}
pub (crate) unsafe fn cpu_halt() {}

pub (crate) unsafe fn interrupt_initialize(_stack: &mut [u8]) {}


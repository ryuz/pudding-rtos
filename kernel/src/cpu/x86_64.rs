
pub unsafe fn cpu_lock() {}

pub unsafe fn cpu_unlock() {}

pub unsafe fn cpu_idle() -> ! {
    loop {}
}


pub unsafe fn interrupt_initialize(_stack: &mut [isize]) {
}




#[repr(C)]
pub struct Context {
    pub sp: usize,
}

#[macro_export]
macro_rules! context_default {
    () => {
        Context { sp: 0 }
    };
}


pub unsafe fn interrupt_initialize(_stack: &mut [isize]) {}

pub unsafe fn cpu_initialize(_stack: &mut [isize]) {}

pub unsafe fn cpu_lock() {}

pub unsafe fn cpu_unlock() {}

pub unsafe fn cpu_halt() {}







#[no_mangle]
pub extern "C" fn _kernel_context_create(
    _ctxcb: *mut Context,
    _isp: usize,
    _entry: extern "C" fn(isize),
    _ext: isize,
) {
}

#[no_mangle]
pub extern "C" fn _kernel_context_start(_ctxcb_new: *mut Context) {}

#[no_mangle]
pub extern "C" fn _kernel_context_switch(
    _ctxcb_new: *mut Context,
    _ctxcb_now: *mut Context,
) {
}

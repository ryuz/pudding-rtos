use super::context::*;

#[no_mangle]
pub extern "C" fn _kernel_context_create(
    _ctxcb: *mut ContextControlBlock,
    _isp: usize,
    _entry: extern "C" fn(isize),
    _ext: isize,
) {
}

#[no_mangle]
pub extern "C" fn _kernel_context_start(_ctxcb_new: *mut ContextControlBlock) {}

#[no_mangle]
pub extern "C" fn _kernel_context_switch(
    _ctxcb_new: *mut ContextControlBlock,
    _ctxcb_now: *mut ContextControlBlock,
) {
}

#![allow(dead_code)]


extern "C" {
    // コンテキスト生成
    fn _kernel_context_create(
        ctxcb: *mut Context,
        isp: usize,
        entry: extern "C" fn(isize),
        exinf: isize,
    );

    // コンテキスト開始
    fn _kernel_context_start(ctxcb_new: *mut Context);

    // コンテキストスイッチ
    fn _kernel_context_switch(ctxcb_next: *mut Context, ctxcb_current: *mut Context);
}


#[repr(C)]
pub struct Context {
    pub sp: usize,
}

impl Context {
    pub const fn new() -> Self {
        Context { sp: 0 }
    }

    pub (crate) unsafe fn _create(&mut self, stack: &mut [u8], entry: extern "C" fn(isize), exinf: isize) {
        let isp = ((&mut stack[0] as *mut u8 as usize) + stack.len()) & 0xfffffff8;
        _kernel_context_create(self as  *mut Context, isp as usize, entry, exinf);
    }

    pub (crate) unsafe fn _start(&mut self)
    {
        _kernel_context_start(self as *mut Context);
    }

    pub (crate) unsafe fn _switch(&mut self, current: &mut Context) {
        _kernel_context_switch(self as *mut Context, current as *mut Context);
    }
}


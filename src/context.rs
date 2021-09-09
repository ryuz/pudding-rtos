use core::mem::size_of;
//use core::mem;

#[repr(C)]
#[derive(Default)]
pub struct ContextControlBlock {
    pub sp: usize,
}

extern "C" {
    // コンテキスト生成
    fn _kernel_context_create(
        ctxcb: *mut ContextControlBlock,
        isp: usize,
        entry: extern "C" fn(isize),
        exinf: isize,
    );

    // コンテキスト開始
    fn _kernel_context_start(ctxcb_new: *mut ContextControlBlock);

    // コンテキストスイッチ
    fn _kernel_context_switch(
        ctxcb_next: *mut ContextControlBlock,
        ctxcb_current: *mut ContextControlBlock,
    );
}


impl ContextControlBlock {
    pub fn new() -> Self {
        ContextControlBlock {sp : 0}
    }

    pub fn create(&mut self, stack: &mut [isize], entry: extern "C" fn(isize), exinf: isize) {
        let isp = (&stack[0] as *const isize as usize) + stack.len() * size_of::<isize>();
        unsafe {
            _kernel_context_create(self, isp as usize, entry, exinf);
        }
    }

    pub fn start(&mut self) {
        unsafe {
            _kernel_context_start(self);
        }
    }

    pub fn switch(&mut self, ctxcb_next: &mut ContextControlBlock) {
        unsafe {
            _kernel_context_switch(ctxcb_next, self);
        }
    }
}

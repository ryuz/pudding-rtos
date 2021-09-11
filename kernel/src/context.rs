use core::mem::size_of;
use core::ptr;

#[repr(C)]
//#[derive(Default)]
pub struct Context {
    pub sp: usize,
}

#[macro_export]
macro_rules! context_default {
    () => {
        Context { sp: 0 }
    };
}

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

static mut SYSTEM_CONTEXT: Context = Context { sp: 0 };
static mut CURRENT_CONTEXT: *mut Context = ptr::null_mut();
//    Lazy::new(|| unsafe { &mut *SYSTEM_CONTEXT as *mut Context });

pub unsafe fn context_switch_system() {
    SYSTEM_CONTEXT.switch();
}

pub fn context_initialize() {
    unsafe {
        CURRENT_CONTEXT = &mut SYSTEM_CONTEXT as *mut Context;
    }
}

impl Context {
    pub fn new() -> Self {
        Context { sp: 0 }
    }

    pub fn create(&mut self, stack: &mut [isize], entry: extern "C" fn(isize), exinf: isize) {
        let isp = (&stack[0] as *const isize as usize) + stack.len() * size_of::<isize>();
        unsafe {
            _kernel_context_create(self, isp as usize, entry, exinf);
        }
    }

    /*
    pub fn start(&mut self) {
        unsafe {
           _kernel_context_start(self);
        }
    }
    */

    pub fn switch(&mut self) {
        unsafe {
            let cur_ctx = CURRENT_CONTEXT;
            CURRENT_CONTEXT = self as *mut Context;
            _kernel_context_switch(self, cur_ctx);
        }
    }

    pub fn is_current(&self) -> bool {
        unsafe {
            let ptr0 = CURRENT_CONTEXT as *const Context;
            let ptr1 = self as *const Context;
            ptr0 == ptr1
        }
    }
}

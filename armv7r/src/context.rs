use core::mem::size_of;
//use core::mem;

#[repr(C)]
#[derive(Default)]
pub struct ContextControlBlock {
    pub sp: usize,
}

extern "C" {
    fn _kernel_context_create(
        ctxcb: *mut ContextControlBlock,
        isp: usize,
        entry: extern "C" fn(isize),
        ext: isize,
    );

    fn _kernel_context_start(
        ctxcb_new: *mut ContextControlBlock,
    );

    fn _kernel_context_switch(
        ctxcb_new: *mut ContextControlBlock,
        ctxcb_now: *mut ContextControlBlock,
    );
}

impl ContextControlBlock {
    pub fn new(stack: &mut [isize], entry: extern "C" fn(isize), ext: isize) -> Self {
        let isp = (&stack[0] as *const isize as usize) + stack.len() * size_of::<isize>();
        //      let mut ctxcb = ContextControlBlock {sp : 0};
        let mut ctxcb: ContextControlBlock = Default::default();
        ctxcb.create(isp, entry, ext);
        ctxcb
    }

    pub fn create(&mut self, isp: usize, entry: extern "C" fn(isize), ext: isize) {
        unsafe {
            _kernel_context_create(self, isp as usize, entry, ext);
        }
    }

    pub fn start(&mut self) {
        unsafe {
            _kernel_context_start(self);
        }
    }

    pub fn switch(&mut self, ctxcb_now: &mut ContextControlBlock) {
        unsafe {
            _kernel_context_switch(ctxcb_now, self);
        }
    }
}

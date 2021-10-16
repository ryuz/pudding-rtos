#[repr(C)]
pub struct Context {
    pub sp: usize,
}

impl Context {
    pub const fn new() -> Self {
        Context { sp: 0 }
    }

    pub(crate) unsafe fn _create(
        &mut self,
        _stack: &mut [u8],
        _entry: extern "C" fn(isize),
        _exinf: isize,
    ) {
    }
    pub(crate) unsafe fn _start(&mut self) {}
    pub(crate) unsafe fn _switch(&mut self, _current: &mut Context) {}
}

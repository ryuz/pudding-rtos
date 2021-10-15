use core::ptr;


#[cfg(target_arch = "x86_64")]
pub mod x86_64;
#[cfg(target_arch = "x86_64")]
pub use x86_64::*;

#[cfg(target_arch = "arm")]
pub mod arm;
#[cfg(target_arch = "arm")]
pub use arm::*;

#[cfg(not(any(target_arch = "x86_64", target_arch = "arm")))]
pub mod dummy;
#[cfg(not(any(target_arch = "x86_64", target_arch = "arm")))]
pub use dummy::*;


static mut SYSTEM_CONTEXT: Context = Context::new();
static mut CURRENT_CONTEXT: *mut Context = ptr::null_mut();


pub(crate) unsafe fn context_switch_to_system() {
    SYSTEM_CONTEXT.switch();
}

pub(crate) fn context_initialize() {
    unsafe {
        CURRENT_CONTEXT = &mut SYSTEM_CONTEXT as *mut Context;
    }
}

impl Context {
    pub (crate) fn create(&mut self, stack: &mut [u8], entry: extern "C" fn(isize), exinf: isize) {
        unsafe {
            self._create(stack, entry, exinf);
        }
    }

    pub (crate) fn switch(&mut self) {
        unsafe {
            let cur_ctx = CURRENT_CONTEXT;
            CURRENT_CONTEXT = self as *mut Context;
            self._switch(&mut *cur_ctx);
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


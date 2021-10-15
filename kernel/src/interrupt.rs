use crate::system::*;
use crate::*;

pub unsafe fn initialize(stack: &mut [u8]) {
    cpu::interrupt_initialize(stack);
}

// 割り込みコンテキストに移行
#[no_mangle]
pub unsafe extern "C" fn _kernel_interrupt_start() {
    set_interrupt_flag();
}

// 割り込みハンドラの実行
#[no_mangle]
pub unsafe extern "C" fn _kernel_interrupt_handler(intno: isize) {
    irc::interrupt_handler(intno);
}

// 割り込みコンテキストを抜けて遅延ディスパッチ実行
#[no_mangle]
pub unsafe extern "C" fn _kernel_interrupt_end() {
    clear_interrupt_flag();
    if test_dispatch_reserve_flag() && !test_dispatch_disable_flag() {
        clear_dispatch_reserve_flag();
        task::task_switch();
    }
}

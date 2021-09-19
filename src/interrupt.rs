


use crate::cpu;
use crate::irc;


pub unsafe fn initialize(stack: &mut [isize])
{
    cpu::interrupt_initialize(stack);
}


// 割り込みコンテキストに移行
#[no_mangle]
pub unsafe extern "C" fn _kernel_interrupt_start() {
}

// 割り込みハンドラの実行
#[no_mangle]
pub unsafe extern "C" fn _kernel_interrupt_handler(intno: isize) {
    irc::interrupt_handler(intno);
}

// 割り込みコンテキストを抜けて遅延ディスパッチ実行
#[no_mangle]
pub unsafe extern "C" fn _kernel_interrupt_end() {
//    task_switch();
}


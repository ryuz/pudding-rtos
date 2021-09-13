

pub type InhNo = i32;

use crate::irc::*;
use crate::task::*;

// 割り込みコンテキストに移行
#[no_mangle]
pub unsafe extern "C" fn _kernel_interrupt_start() {

}

// 割り込みハンドラの実行
#[no_mangle]
pub unsafe extern "C" fn _kernel_interrupt_handler(intno: isize) {
    interrupt_handler(intno);
}

// 割り込みコンテキストを抜けて遅延ディスパッチ実行
#[no_mangle]
pub unsafe extern "C" fn _kernel_interrupt_end() {
    task_switch();
}

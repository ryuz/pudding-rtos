

#[cfg(feature="pl390")]
pub mod pl390;
#[cfg(feature="pl390")]
pub use pl390::*;


#[cfg(not(feature="pl390"))]
pub mod dummy;

#[cfg(not(feature="pl390"))]
pub use dummy::*;



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
//    task_switch();
}


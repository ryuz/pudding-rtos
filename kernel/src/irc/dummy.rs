pub type IntNo = i32;

// 割込みの許可
pub fn interrupt_enable(_intno: IntNo) {}
pub fn interrupt_disable(_intno: IntNo) {}

// 割り込みハンドラ登録
pub unsafe fn interrupt_set_handler(_intno: IntNo, _handler: Option<fn()>) {}

// 割込みコントローラの割込み処理
pub(in crate) unsafe fn interrupt_handler(_inhno: isize) {}

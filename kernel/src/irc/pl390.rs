#![allow(dead_code)]

use jelly_pac::arm::pl390::Pl390;

static mut PL390: Pl390 = Pl390 { icc: 0, icd: 0 };

pub fn take() -> &'static mut Pl390 {
    unsafe { &mut PL390 }
}

pub fn initialize(icc_address: usize, icd_address: usize) {
    unsafe {
        PL390.set_base_address(icc_address, icd_address);
        PL390.initialize();
    }
}

// 割込みの禁止
pub fn interrupt_disable(intno: usize) {
    unsafe {
        PL390.interrupt_disable(intno);
    }
}

// 割込みの許可
pub fn interrupt_enable(intno: usize) {
    unsafe {
        PL390.interrupt_enable(intno);
    }
}

// 割込み優先度変更
pub fn interrupt_set_priority(intno: usize, pri: u8) {
    unsafe {
        PL390.interrupt_set_priority(intno, pri);
    }
}

// 割り込みハンドラ登録
static mut INTERRUPT_HANDLERS: [Option<fn()>; 255] = [None; 255];

pub unsafe fn interrupt_set_handler(intno: usize, handler: Option<fn()>) {
    INTERRUPT_HANDLERS[intno] = handler;
}

// 割込みコントローラの割込み処理
pub(in crate) unsafe fn interrupt_handler(_: isize) {
    // 割込み番号取得
    let icciar = PL390.read_icciar();
    let intno = icciar as usize;

    // 優先度マスク更新
    let pmr = PL390.read_iccpmr();
    let ilv = PL390.read_icdipr(intno);
    PL390.write_iccpmr(ilv);

    // 先に割り込みを終わらせる
    PL390.write_icceoir(icciar);

    // 割込みサービスルーチン呼び出し
    //    _kernel_ictxcb.imsk &= ~_KERNEL_IMSK_I;	// 多重割り込み許可
    //    _kernel_exe_isr((INTNO)intno);
    INTERRUPT_HANDLERS[intno].unwrap()();
    //    _kernel_ictxcb.imsk |= _KERNEL_IMSK_I;

    // 優先度マスク復帰
    PL390.write_iccpmr(pmr);
}

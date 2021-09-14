#![allow(dead_code)]


use crate::register::*;

pub type IntNo = i32;


// メモリマップドレジスタ定義
const ICCICR: usize = 0x000; // CPU インタフェース制御レジスタ
const ICCPMR: usize = 0x004; // 割り込み優先度マスクレジスタ
const ICCBPR: usize = 0x008; // 2進小数点レジスタ
const ICCIAR: usize = 0x00C; // 割り込み応答レジスタ
const ICCEOIR: usize = 0x010; // 割り込み終了レジスタ
const ICCRPR: usize = 0x014; // 実行優先度レジスタ
const ICCHPIR: usize = 0x018; // 最優先保留割り込みレジスタ
const ICCABPR: usize = 0x01c; // エイリアスされた非セキュア2 進小数点レジスタ
const ICCIDR: usize = 0x0fc; // CPUインタフェース実装識別レジスタ

const ICDDCR: usize = 0x000; // 配器制御レジスタ
const ICDICTR: usize = 0x004; // 割り込みコントローラタイプ レジスタ
const ICDIIDR: usize = 0x008; // 分配器実装者識別レジスタ
const ICDISR: usize = 0x080; // 割り込みセキュリティレジスタ
const ICDISER: usize = 0x100; // 割り込みイネーブルセットレジスタ
const ICDICER: usize = 0x180; // 割り込みイネーブルクリアレジスタ
const ICDISPR: usize = 0x200; // 割り込み保留セットレジスタ
const ICDICPR: usize = 0x280; // 割り込み保留クリアレジスタ
const ICDABR: usize = 0x300; // アクティブビット レジスタ
const ICDIPR: usize = 0x400; // 割り込み優先度レジスタ
const ICDIPTR: usize = 0x800; // 割り込みプロセッサターゲットレジスタ
const ICDICFR: usize = 0xc00; // 割り込み構成レジスタ
const ICDSGIR: usize = 0xf00; // ソフトウェア生成割り込みレジスタ

static mut ICC_ADDRESS: usize = 0;
static mut ICD_ADDRESS: usize = 0;

// CPU インタフェース制御レジスタ
unsafe fn write_iccicr(data: u32) {
    reg32_write(ICC_ADDRESS + ICCICR, data);
}

// 割り込み優先度マスクレジスタ
unsafe fn write_iccpmr(data: u8) {
    reg8_write(ICC_ADDRESS + ICCPMR, data);
}
unsafe fn read_iccpmr() -> u8 {
    reg8_read(ICC_ADDRESS + ICCPMR)
}

// 2進小数点レジスタ
unsafe fn write_iccbpr(data: u32) {
    reg32_write(ICC_ADDRESS + ICCBPR, data);
}

// 割り込み応答レジスタ
unsafe fn write_icciar(data: u32) {
    reg32_write(ICC_ADDRESS + ICCIAR, data);
}
unsafe fn read_icciar() -> u32 {
    reg32_read(ICC_ADDRESS + ICCIAR)
}

// 割り込み終了レジスタ
unsafe fn write_icceoir(data: u32) {
    reg32_write(ICC_ADDRESS + ICCEOIR, data);
}

// 実行優先度レジスタ
unsafe fn write_iccrpr(data: u32) {
    reg32_write(ICC_ADDRESS + ICCRPR, data);
}

// 最優先保留割り込みレジスタ
unsafe fn write_icchpir(data: u32) {
    reg32_write(ICC_ADDRESS + ICCHPIR, data);
}

// エイリアスされた非セキュア2 進小数点レジスタ
unsafe fn write_iccabpr(data: u32) {
    reg32_write(ICC_ADDRESS + ICCABPR, data);
}

// CPUインタフェース実装識別レジスタ
unsafe fn write_iccidr(data: u32) {
    reg32_write(ICC_ADDRESS + ICCIDR, data);
}

// 配器制御レジスタ
unsafe fn write_icddcr(data: u32) {
    reg32_write(ICD_ADDRESS + ICDDCR, data);
}

// 割り込みコントローラタイプ レジスタ
unsafe fn write_icdictr(data: u32) {
    reg32_write(ICD_ADDRESS + ICDICTR, data);
}

// 分配器実装者識別レジスタ
unsafe fn write_icdiidr(data: u32) {
    reg32_write(ICD_ADDRESS + ICDIIDR, data);
}

// 割り込みセキュリティレジスタ
unsafe fn write_icdisr(n: usize, data: u32) {
    reg32_write(ICD_ADDRESS + ICDISR + 4 * n, data);
}

// 割り込みイネーブルセットレジスタ */
unsafe fn write_icdiser(n: usize, data: u32) {
    reg32_write(ICD_ADDRESS + ICDISER + 4 * n, data);
}

// 割り込みイネーブルクリアレジスタ */
unsafe fn write_icdicer(n: usize, data: u32) {
    reg32_write(ICD_ADDRESS + ICDICER + 4 * n, data);
}

// 割り込み保留セットレジスタ */
unsafe fn write_icdispr(n: usize, data: u32) {
    reg32_write(ICD_ADDRESS + ICDISPR + 4 * n, data);
}

// 割り込み保留クリアレジスタ */
unsafe fn write_icdicpr(n: usize, data: u32) {
    reg32_write(ICD_ADDRESS + ICDICPR + 4 * n, data);
}

// アクティブビット レジスタ
unsafe fn write_icdabr(n: usize, data: u32) {
    reg32_write(ICD_ADDRESS + ICDABR + 4 * n, data);
}

// 割り込み優先度レジスタ
unsafe fn write_icdipr(n: usize, data: u8) {
    reg8_write(ICD_ADDRESS + ICDIPR + n, data);
}
unsafe fn read_icdipr(n: usize) -> u8 {
    reg8_read(ICD_ADDRESS + ICDIPR + n)
}

// 割り込みプロセッサターゲットレジスタ
unsafe fn write_icdiptr(n: usize, data: u8) {
    reg8_write(ICD_ADDRESS + ICDIPTR + n, data);
}

// 割り込み構成レジスタ
unsafe fn write_icdicfr(n: usize, data: u32) {
    reg32_write(ICD_ADDRESS + ICDICFR + 4 * n, data);
}
unsafe fn read_icdicfr(n: usize) -> u32 {
    reg32_read(ICD_ADDRESS + ICDICFR + 4 * n)
}

// ソフトウェア生成割り込みレジスタ
unsafe fn write_icdsgir(data: u32) {
    reg32_write(ICD_ADDRESS + ICDSGIR, data);
}




pub unsafe fn icd_enable() {
    write_icddcr(1);
}

pub unsafe fn icd_disable() {
    write_icddcr(0);
}

/// ICDIPTR(target CPU)
pub unsafe fn icd_set_target(intno: IntNo, targetcpu: u8)
{
	write_icdiptr(intno as usize, targetcpu);
}


pub unsafe fn icd_set_config(intno: IntNo, config: u8)
{
    let n = intno as usize >> 4;
    let s = (intno as u32 & 0x0f) * 2;

    let mut val = read_icdicfr(n);
    val &= !(0x03 << s);
    val |= (config as u32 & 0x03) << s;
    write_icdicfr(n, val);
}


pub fn initialize(icc_address: usize, idc_address: usize) {
    unsafe {
        ICC_ADDRESS = icc_address;
        ICD_ADDRESS = idc_address;

        write_iccpmr(0xf8);
        write_iccicr(0x07);
    }
}

// 割込みの禁止
pub fn interrupt_disable(intno: IntNo) {
    unsafe {
        write_icdicer(intno as usize >> 5, 1 << (intno & 0x1f));
    }
}

// 割込みの許可
pub fn interrupt_enable(intno: IntNo) {
    unsafe {
        write_icdiser(intno as usize >> 5, 1 << (intno & 0x1f));
    }
}

// 割込み優先度変更
pub fn interrupt_set_priority(intno: IntNo, pri: u8)
{
    unsafe {
        write_icdipr(intno as usize, pri);
    }
}


// 割り込みハンドラ登録
static mut INTERRUPT_HANDLERS: [Option<fn()>; 255] = [None; 255];

pub unsafe fn interrupt_set_handler(intno: IntNo, handler: Option<fn()>) {
    INTERRUPT_HANDLERS[intno as usize] = handler;
}



// 割込みコントローラの割込み処理
pub (in crate) unsafe fn interrupt_handler(_: isize) {
    // 割込み番号取得
    let icciar = read_icciar();
    let intno = icciar as usize;
//    if intno >= INTERRUPT_HANDLERS.len() {
//        panic!("unexpected ICCIAR");
//    }

    // 優先度マスク更新
    let pmr = read_iccpmr();
    let ilv = read_icdipr(intno);
    write_iccpmr(ilv);

    // 先に割り込みを終わらせる
    write_icceoir(icciar);

    // 割込みサービスルーチン呼び出し
    //    _kernel_ictxcb.imsk &= ~_KERNEL_IMSK_I;	// 多重割り込み許可
    //    _kernel_exe_isr((INTNO)intno);
    INTERRUPT_HANDLERS[intno].unwrap()();
    //    _kernel_ictxcb.imsk |= _KERNEL_IMSK_I;

    // 優先度マスク復帰
    write_iccpmr(pmr);
}


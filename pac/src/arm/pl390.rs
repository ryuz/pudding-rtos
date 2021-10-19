#![allow(dead_code)]

// https://ryuz.hatenablog.com/entry/2021/04/03/194046

use core::ptr;

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

// レジスタ書き込み
unsafe fn reg8_write(addr: usize, data: u8) {
    let p = addr as *mut u8;
    ptr::write_volatile(p, data);
}

// レジスタ読み出し
unsafe fn reg8_read(addr: usize) -> u8 {
    let p = addr as *mut u8;
    ptr::read_volatile(p)
}

// レジスタ書き込み
unsafe fn reg32_write(addr: usize, data: u32) {
    let p = addr as *mut u32;
    ptr::write_volatile(p, data);
}

// レジスタ読み出し
unsafe fn reg32_read(addr: usize) -> u32 {
    let p = addr as *mut u32;
    ptr::read_volatile(p)
}

// PL390
pub struct Pl390 {
    pub icc: usize,
    pub icd: usize,
}

impl Pl390 {
    // 生成
    pub const fn new(icc: usize, icd: usize) -> Self {
        Pl390 { icc: icc, icd: icd }
    }
    
    // ベースアドレス設定
    pub fn set_base_address(&mut self, icc: usize, icd: usize) {
        self.icc = icc;
        self.icd = icd;
    }

    // 初期化
    pub unsafe fn initialize(&self) {
        self.write_iccpmr(0xf8);
        self.write_iccicr(0x07);
    }

    // ICD有効化
    pub unsafe fn icd_enable(&self) {
        self.write_icddcr(1);
    }

    // ICD無効化
    pub unsafe fn icd_disable(&self) {
        self.write_icddcr(0);
    }

    // ターゲットCPU設定
    pub unsafe fn icd_set_target(&self, intno: usize, targetcpu: u8) {
        self.write_icdiptr(intno, targetcpu);
    }

    // ターゲットCPU設定
    pub unsafe fn icd_set_config(&self, intno: usize, config: u8) {
        let n = intno as usize >> 4;
        let s = (intno as u32 & 0x0f) * 2;

        let mut val = self.read_icdicfr(n);
        val &= !(0x03 << s);
        val |= (config as u32 & 0x03) << s;
        self.write_icdicfr(n, val);
    }

    // 割込みの禁止
    pub fn interrupt_disable(&self, intno: usize) {
        unsafe {
            self.write_icdicer(intno >> 5, 1 << (intno & 0x1f));
        }
    }

    // 割込みの許可
    pub fn interrupt_enable(&self, intno: usize) {
        unsafe {
            self.write_icdiser(intno as usize >> 5, 1 << (intno & 0x1f));
        }
    }

    // 割込み優先度変更
    pub fn interrupt_set_priority(&self, intno: usize, pri: u8) {
        unsafe {
            self.write_icdipr(intno, pri);
        }
    }

    // 割込み保留クリア
    pub fn interrupt_pending_clear(&self, intno: usize) {
        unsafe {
            self.write_icdicpr(intno / 32, 1u32 << (intno % 32));
        }
    }

    // ----- レジスタアクセス -----

    // CPU インタフェース制御レジスタ
    pub unsafe fn write_iccicr(&self, data: u32) {
        reg32_write(self.icc + ICCICR, data);
    }

    // 割り込み優先度マスクレジスタ
    pub unsafe fn write_iccpmr(&self, data: u8) {
        reg8_write(self.icc + ICCPMR, data);
    }
    pub unsafe fn read_iccpmr(&self) -> u8 {
        reg8_read(self.icc + ICCPMR)
    }

    // 2進小数点レジスタ
    pub unsafe fn write_iccbpr(&self, data: u32) {
        reg32_write(self.icc + ICCBPR, data);
    }

    // 割り込み応答レジスタ
    pub unsafe fn write_icciar(&self, data: u32) {
        reg32_write(self.icc + ICCIAR, data);
    }
    pub unsafe fn read_icciar(&self) -> u32 {
        reg32_read(self.icc + ICCIAR)
    }

    // 割り込み終了レジスタ
    pub unsafe fn write_icceoir(&self, data: u32) {
        reg32_write(self.icc + ICCEOIR, data);
    }

    // 実行優先度レジスタ
    pub unsafe fn write_iccrpr(&self, data: u32) {
        reg32_write(self.icc + ICCRPR, data);
    }

    // 最優先保留割り込みレジスタ
    pub unsafe fn write_icchpir(&self, data: u32) {
        reg32_write(self.icc + ICCHPIR, data);
    }

    // エイリアスされた非セキュア2 進小数点レジスタ
    pub unsafe fn write_iccabpr(&self, data: u32) {
        reg32_write(self.icc + ICCABPR, data);
    }

    // CPUインタフェース実装識別レジスタ
    pub unsafe fn write_iccidr(&self, data: u32) {
        reg32_write(self.icc + ICCIDR, data);
    }

    // 配器制御レジスタ
    pub unsafe fn write_icddcr(&self, data: u32) {
        reg32_write(self.icd + ICDDCR, data);
    }

    // 割り込みコントローラタイプ レジスタ
    pub unsafe fn write_icdictr(&self, data: u32) {
        reg32_write(self.icd + ICDICTR, data);
    }

    // 分配器実装者識別レジスタ
    pub unsafe fn write_icdiidr(&self, data: u32) {
        reg32_write(self.icd + ICDIIDR, data);
    }

    // 割り込みセキュリティレジスタ
    pub unsafe fn write_icdisr(&self, n: usize, data: u32) {
        reg32_write(self.icd + ICDISR + 4 * n, data);
    }

    // 割り込みイネーブルセットレジスタ
    pub unsafe fn write_icdiser(&self, n: usize, data: u32) {
        reg32_write(self.icd + ICDISER + 4 * n, data);
    }

    // 割り込みイネーブルクリアレジスタ
    pub unsafe fn write_icdicer(&self, n: usize, data: u32) {
        reg32_write(self.icd + ICDICER + 4 * n, data);
    }

    // 割り込み保留セットレジスタ
    pub unsafe fn write_icdispr(&self, n: usize, data: u32) {
        reg32_write(self.icd + ICDISPR + 4 * n, data);
    }

    // 割り込み保留クリアレジスタ
    pub unsafe fn write_icdicpr(&self, n: usize, data: u32) {
        reg32_write(self.icd + ICDICPR + 4 * n, data);
    }

    // アクティブビット レジスタ
    pub unsafe fn write_icdabr(&self, n: usize, data: u32) {
        reg32_write(self.icd + ICDABR + 4 * n, data);
    }

    // 割り込み優先度レジスタ
    pub unsafe fn write_icdipr(&self, n: usize, data: u8) {
        reg8_write(self.icd + ICDIPR + n, data);
    }
    pub unsafe fn read_icdipr(&self, n: usize) -> u8 {
        reg8_read(self.icd + ICDIPR + n)
    }

    // 割り込みプロセッサターゲットレジスタ
    pub unsafe fn write_icdiptr(&self, n: usize, data: u8) {
        reg8_write(self.icd + ICDIPTR + n, data);
    }

    // 割り込み構成レジスタ
    pub unsafe fn write_icdicfr(&self, n: usize, data: u32) {
        reg32_write(self.icd + ICDICFR + 4 * n, data);
    }
    pub unsafe fn read_icdicfr(&self, n: usize) -> u32 {
        reg32_read(self.icd + ICDICFR + 4 * n)
    }

    // ソフトウェア生成割り込みレジスタ
    pub unsafe fn write_icdsgir(&self, data: u32) {
        reg32_write(self.icd + ICDSGIR, data);
    }
}

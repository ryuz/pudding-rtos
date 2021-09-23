#![allow(dead_code)]
#![cfg(target_arch = "arm")]

// T_Bit  0x20
// F_Bit  0x40
// I_Bit  0x80

pub unsafe fn fiq_enable() {
    asm!(
        r#"
            mrs     r0, cpsr                    /* cpsr取得 */
            bic     r0, r0, #0x40               /* Fビットをクリア */
            msr     cpsr_c, r0                  /* cpsr設定 */
        "#
    );
}

pub unsafe fn fiq_disable() {
    asm!(
        r#"
            mrs     r0, cpsr                    /* cpsr取得 */
            orr     r0, r0, #0x40               /* Fビットを設定 */
            msr     cpsr_c, r0                  /* cpsr設定 */
        "#
    );
}

pub unsafe fn irq_enable() {
    asm!(
        r#"
            mrs     r0, cpsr                    /* cpsr取得 */
            bic     r0, r0, #0x80               /* Iビットをクリア */
            msr     cpsr_c, r0                  /* cpsr設定 */
        "#
    );
}

pub unsafe fn irq_disable() {
    asm!(
        r#"
            mrs     r0, cpsr                    /* cpsr取得 */
            orr     r0, r0, #0x80               /* Iビットを設定 */
            msr     cpsr_c, r0                  /* cpsr設定 */
        "#
    );
}

pub unsafe fn int_enable() {
    asm!(
        r#"
            mrs     r0, cpsr                    /* cpsr取得 */
            bic     r0, r0, #(0x40 | 0x80)      /* FビットとIビットをクリア */
            msr     cpsr_c, r0                  /* cpsr設定 */
        "#
    );
}

pub unsafe fn int_disable() {
    asm!(
        r#"
            mrs     r0, cpsr                    /* cpsr取得 */
            orr     r0, r0, #(0x40 | 0x80)      /* FビットとIビットを設定 */
            msr     cpsr_c, r0                  /* cpsr設定 */
        "#
    );
}

/// 割り込み待ち
pub unsafe fn wfi() -> ! {
    loop {
        asm!("wfi");
    }
}

///  分岐予測有効化
pub unsafe fn enable_bpredict() {
    asm!(
        r#"
            mrc     p15, 0, r0, c1, c0, 1       /* Read ACTLR */
            bic     r0, r0, #0x1 << 17          /* Clear RSDIS bit 17 to enable return stack */
            bic     r0, r0, #0x1 << 16          /* Clear BP bit 15 and BP bit 16 */
            bic     r0, r0, #0x1 << 15          /* Normal operation, BP is taken from the global history table */
            mcr     p15, 0, r0, c1, c0, 1       /* Write ACTLR */
            dsb
        "#
    );
}

/// 分岐予測無効化
pub unsafe fn disable_bpredict() {
    asm!(
        r#"
            mrc     p15, 0, r0, c1, c0, 1       /* Read ACTLR */
            orr     r0, r0, #0x1 << 17          /* Enable RSDIS bit 17 to disable the return stack */
            orr     r0, r0, #0x1 << 16          /* Clear BP bit 15 and set BP bit 16:*/
            bic     r0, r0, #0x1 << 15          /* Branch always not taken and history table updates disabled*/
            mcr     p15, 0, r0, c1, c0, 1       /* Write ACTLR */
            dsb
        "#
    );
}

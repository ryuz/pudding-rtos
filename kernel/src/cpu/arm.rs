

pub mod cp;
pub mod mpu;


#[repr(C)]
pub struct CpuControlBlock {
    pub imask: u32, // 割り込みマスク状態
    pub inest: u32, // 多重割り込みネスト回数
    pub isp: u32,   // 割り込みスタック初期値
}

static mut CPU_CB: CpuControlBlock = CpuControlBlock {
    imask: 0,
    inest: 0,
    isp: 0,
};


pub unsafe fn cpu_initialize() {
}


pub unsafe fn interrupt_initialize(stack: &mut [isize]) {
    let isp = (&stack[0] as *const isize as usize) + stack.len() * core::mem::size_of::<isize>();
    CPU_CB.isp = isp as u32;
}



pub unsafe fn cpu_lock() {
    asm!(
        r#"
            mrs     r0, cpsr                    /* cpsr取得 */
            orr     r0, r0, #(0x40 | 0x80)      /* FビットとIビットを設定 */
            msr     cpsr_c, r0                  /* cpsr設定 */
        "#
    );
}

pub unsafe fn cpu_unlock() {
    let imask = CPU_CB.imask;
    asm!(
        r#"
            mrs     r0, cpsr                    /* cpsr取得 */
            bic     r0, r0, #(0x40 | 0x80)      /* FビットとIビットをクリア */
            orr     r0, r0, {0}                 /* 割込みマスク設定 */
            msr     cpsr_c, r0                  /* cpsr設定 */
        "#,
        in(reg) imask,
    );
}


/// 割り込み待ち
pub unsafe fn cpu_idle() -> ! {
    loop {
        asm!("wfi");
    }
}

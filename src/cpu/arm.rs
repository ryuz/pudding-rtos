#![allow(dead_code)]

pub use pudding_pac::arm::cpu;
pub use pudding_pac::arm::mpu;
pub use pudding_pac::arm::vfp;

#[repr(C)]
struct CpuControlBlock {
    pub imask: u32, // 割り込みマスク状態
    pub inest: u32, // 多重割り込みネスト回数
    pub isp: u32,   // 割り込みスタック初期値
}

#[no_mangle]
static mut _KERNEL_CPU_CB: CpuControlBlock = CpuControlBlock {
    imask: 0,
    inest: 0,
    isp: 0,
};

pub(crate) unsafe fn cpu_initialize() {}

pub(crate) unsafe fn interrupt_initialize(stack: &mut [u8]) {
    let isp = (&mut stack[0] as *mut u8 as usize) + stack.len();
    _KERNEL_CPU_CB.isp = (isp as u32) & 0xfffffff0;
}

pub(crate) unsafe fn cpu_lock() {
    asm!(
        r#"
            mrs     r0, cpsr                    /* cpsr取得 */
            orr     r0, r0, #(0x40 | 0x80)      /* FビットとIビットを設定 */
            msr     cpsr_c, r0                  /* cpsr設定 */
        "#
    );
}

pub(crate) unsafe fn cpu_unlock() {
    let imask = _KERNEL_CPU_CB.imask;
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

pub(crate) unsafe fn cpu_halt() -> ! {
    loop {
        asm!("wfi");
    }
}

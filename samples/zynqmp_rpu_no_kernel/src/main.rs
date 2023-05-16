#![no_std]
#![no_main]
//#![feature(asm)]
#![allow(stable_features)]

use pudding_pac::arm::cpu;
use pudding_pac::arm::pl390::Pl390;

mod bootstrap;

#[macro_use]
mod uart;
use uart::*;
mod memdump;
mod timer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    println!("\r\n!!!panic!!!");
    loop {}
}

// 割り込みコントローラ
static mut PL390: Pl390 = Pl390 {
    icc: 0xf9001000,
    icd: 0xf9000000,
};

// main
#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    wait(10000);
    println!("Hello world!");

    // タイマ初期化
    timer::timer_initialize();
    irq_initialize();

    /*
        println!("---- ICC --------");
        memdump::memdump(0xf9001000, 256*4);
        println!("---- ICD --------");
        memdump::memdump(0xf9000000, 256*4);
        loop {}
    */
    PL390.write_icceoir(74);

    timer::timer_start();

    cpu::irq_enable();

    loop {
        wait(1000000);
        let time = timer::timer_get_counter_value() as f32 / 100000000.0;
        println!("timer counter:{} [s]", time);
    }
}

// ループによるウェイト
fn wait(n: i32) {
    let mut v: i32 = 0;
    for i in 1..n {
        unsafe { core::ptr::write_volatile(&mut v, i) };
    }
}

// 割り込みハンドラ
#[no_mangle]
pub unsafe extern "C" fn irq_handler() {
    // 割込み番号取得
    let icciar = PL390.read_icciar();

    match icciar {
        74 => {
            timer_handler();
        }
        _ => (),
    }

    // 割り込みを終わらせる
    PL390.write_icceoir(icciar);
}

// タイマ割込みハンドラ
pub fn timer_handler() {
    timer::timer_int_clear();
    println!("timer irq");
}

// 割り込み関連の初期化
unsafe fn irq_initialize() {
    let pl390 = &mut PL390;

    // 初期化
    pl390.initialize();

    // ICD 設定
    let targetcpu: u8 = 0x01;
    pl390.icd_disable();
    pl390.icd_set_target(74, targetcpu); // set TTC0-1
    for i in 0..8 {
        pl390.icd_set_target(121 + i, targetcpu); // PL irq0[7:0]
        pl390.icd_set_config(121 + i, 0x01); // 0x01: level, 0x03: edge
    }
    for i in 0..8 {
        pl390.icd_set_target(136 + i, targetcpu); // PL irq1[7:0]
        pl390.icd_set_config(136 + i, 0x01); // 0x01: level, 0x03: edge
    }
    pl390.icd_enable();

    // タイマ割り込み許可
    pl390.interrupt_set_priority(74, 0xa0);
    pl390.interrupt_enable(74);
}

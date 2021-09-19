#![no_std]
#![no_main]
#![feature(asm)]

use driver::arm::pl390::Pl390;
use driver::arm::cpu;

mod bootstrap;

#[macro_use]
mod uart;
use uart::*;
mod timer;

use core::panic::PanicInfo;


#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    println!("\r\n!!!panic!!!");
    loop {}
}

// main
#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    wait(10000);
    println!("Hello world");

    initialize_irq();
    timer::timer_initialize();

    cpu::irq_enable();
//  cpu::fiq_enable();

    loop {
        wait(1000000);
        println!("{}", timer::timer_get_counter_value());
    }
}


fn wait(n: i32) {
    let mut v: i32 = 0;
    for i in 1..n {
        unsafe { core::ptr::write_volatile(&mut v, i) };
    }
}


// 割り込みコントローラ
static mut PL390:Pl390 = Pl390 {icc:0xf9001000, icd:0xf9000000};


#[no_mangle]
pub unsafe extern "C" fn _irq_handler() {
    // 割込み番号取得
    let icciar = PL390.read_icciar();

    match icciar {
        74 => { timer::timer_int_handler(); },
        _ => (),
    }

    // 割り込みを終わらせる
    PL390.write_icceoir(icciar);
}


unsafe fn initialize_irq() {
    let pl390 = &mut PL390;

    pl390.initialize();

    let targetcpu: u8 = 0x01;
    pl390.icd_disable();

    // set TTC0-1
    pl390.icd_set_target(74, targetcpu);
    
    // PL
    for i in 0..8 {
        pl390.icd_set_target(121 + i, targetcpu);
        pl390.icd_set_config(121 + i, 0x01);       // 0x01: level, 0x03: edge
    }
    for i in 0..8 {
        pl390.icd_set_target(136 + i, targetcpu);
        pl390.icd_set_config(136 + i, 0x01);       // 0x01: level, 0x03: edge
    }
    
    pl390.icd_enable(); 

    // タイマ割り込み許可
    pl390.interrupt_set_priority(74, 0xa0);
    pl390.interrupt_enable(74);
}



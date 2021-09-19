
#![allow(dead_code)]

use driver::cdns::ttc::Ttc;


// TTC0 : 0xFF110000 irq:68-70
// TTC1 : 0xFF120000 irq:71-73
// TTC2 : 0xFF130000 irq:74-76
// TTC3 : 0xFF140000 irq:77-79
const TTC_ADDRESS :usize = 0xff130000;
const TTC_INTNO   :usize = 74;


// OS用タイマ初期化ルーチン
#[no_mangle]
pub unsafe fn timer_initialize() {
    let ttc = &mut *(TTC_ADDRESS as *mut Ttc);

    // タイマ動作開始
    core::ptr::write_volatile(&mut ttc.counter_control_1, 0x31); // stop and reset
    core::ptr::write_volatile(&mut ttc.counter_control_1, 0x21); // stop

    core::ptr::write_volatile(&mut ttc.clock_control_1, 0x03); // PS_VAL:1, PS_EN:1
    core::ptr::write_volatile(&mut ttc.interval_counter_1, 25000 - 1); // 1kHz (CPU_1x:100MHz)
//  core::ptr::write_volatile(&mut ttc.interval_counter_1, 250000 - 1); // 1Hz (CPU_1x:100MHz)

    core::ptr::write_volatile(&mut ttc.interrupt_register_1, 0x01); // Interrupt : Interval
    core::ptr::write_volatile(&mut ttc.interrupt_enable_1, 0x01); // Interrupt enable

    core::ptr::write_volatile(&mut ttc.counter_control_1, 0x22); // start
}

pub fn timer_get_counter_value() -> u32 {
    unsafe {
        let ttc = &mut *(TTC_ADDRESS as *mut Ttc);
        core::ptr::read_volatile(&mut ttc.counter_value_1)
    }
}

static mut TIMER_COUNTER: u32 = 0;

// タイマ割込みハンドラ
pub fn timer_int_handler() {
    unsafe {
        let ttc = &mut *(TTC_ADDRESS as *mut Ttc);
        
        //  割込み要因クリア
        core::ptr::read_volatile(&mut ttc.interrupt_register_1); // 読み出すとクリア
        println!("timer irq");
        
        TIMER_COUNTER = TIMER_COUNTER.wrapping_add(1);
        if TIMER_COUNTER % 1000 == 0 {
            println!("timer irq");
        }
    }
}


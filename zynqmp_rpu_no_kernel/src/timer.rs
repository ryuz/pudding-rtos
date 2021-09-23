#![allow(dead_code)]

use jelly_pac::cdns::ttc::Ttc;

// TTC0 : 0xFF110000 irq:68-70
// TTC1 : 0xFF120000 irq:71-73
// TTC2 : 0xFF130000 irq:74-76
// TTC3 : 0xFF140000 irq:77-79
const TTC_ADDRESS: usize = 0xff130000;
const TTC_INTNO: usize = 74;

// OS用タイマ初期化ルーチン
pub unsafe fn timer_initialize() {
    let ttc = &mut *(TTC_ADDRESS as *mut Ttc);

    // タイマ停止
    core::ptr::write_volatile(&mut ttc.counter_control_1, 0x31); // stop and reset
    core::ptr::write_volatile(&mut ttc.counter_control_1, 0x21); // stop

    core::ptr::write_volatile(&mut ttc.counter_control_2, 0x31); // stop and reset
    core::ptr::write_volatile(&mut ttc.counter_control_2, 0x21); // stop

    // 割り込み停止
    core::ptr::read_volatile(&mut ttc.interrupt_register_1); // 読み出すとクリア
    core::ptr::write_volatile(&mut ttc.interrupt_register_1, 0x00); // Interrupt : Interval
    core::ptr::write_volatile(&mut ttc.interrupt_enable_1, 0x00); // Interrupt disable
}

// タイマ動作開始
pub unsafe fn timer_start() {
    let ttc = &mut *(TTC_ADDRESS as *mut Ttc);

    core::ptr::write_volatile(&mut ttc.clock_control_1, 0x03); // PS_VAL:1, PS_EN:1
    core::ptr::write_volatile(&mut ttc.interval_counter_1, 25000000 - 1); // 1Hz (CPU_1x:100MHz->25MHz)

    core::ptr::write_volatile(&mut ttc.interrupt_register_1, 0x01); // Interrupt : Interval
    core::ptr::write_volatile(&mut ttc.interrupt_enable_1, 0x01); // Interrupt enable

    core::ptr::write_volatile(&mut ttc.counter_control_1, 0x22); // start (interval timer)

    core::ptr::write_volatile(&mut ttc.clock_control_2, 0x00); // PS_VAL:1, PS_EN:1
    core::ptr::write_volatile(&mut ttc.counter_control_2, 0x20); // start (free run)
}

pub fn timer_get_counter_value() -> u32 {
    unsafe {
        let ttc = &mut *(TTC_ADDRESS as *mut Ttc);
        core::ptr::read_volatile(&mut ttc.counter_value_2)
    }
}

pub fn timer_int_clear() {
    unsafe {
        let ttc = &mut *(TTC_ADDRESS as *mut Ttc);
        core::ptr::read_volatile(&mut ttc.interrupt_register_1); // //  割込み要因クリア(読み出すとクリア)
    }
}

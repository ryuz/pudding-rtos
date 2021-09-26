#![allow(dead_code)]

use jelly_kernel as kernel;
use jelly_pac::cdns::ttc::*;

// TTC0 : 0xFF110000 irq:68-70
// TTC1 : 0xFF120000 irq:71-73
// TTC2 : 0xFF130000 irq:74-76
// TTC3 : 0xFF140000 irq:77-79
const TTC_ADDRESS: usize = 0xff130000;
const TTC_INTNO: usize = 74;

static mut TTC: Ttc = Ttc {
    address: TTC_ADDRESS,
};

pub fn timer_initialize(timer_int_handler: fn()) {
    unsafe {
        kernel::irc::interrupt_set_handler(TTC_INTNO, Some(timer_int_handler));
        kernel::irc::interrupt_set_priority(TTC_INTNO, 0xa0);
        kernel::irc::interrupt_enable(TTC_INTNO);

        // timer1 (interval timeer)
        TTC.reset(Timer::Timer1);
        TTC.set_clock_control(Timer::Timer1, ClockControl::PRESCALER_ENABLE, 1);
        TTC.set_interval_counter(Timer::Timer1, 25000 - 1); // 1Hz (CPU_1x:100MHz->25MHz)

        TTC.enable_interrupt(Timer::Timer1, Interrupt::INTERVAL);
        TTC.set_counter_control(
            Timer::Timer1,
            CounterControl::INTERVAL | CounterControl::OUTPUT_WAVEFORM_DISABLE,
        );

        // timer2 (free run counter)
        TTC.reset(Timer::Timer2);
        TTC.set_clock_control(Timer::Timer2, ClockControl::NONE, 0);
        TTC.set_counter_control(Timer::Timer2, CounterControl::OUTPUT_WAVEFORM_DISABLE);
    }
}

pub fn timer_get_counter_value() -> u32 {
    unsafe { TTC.get_counter_value(Timer::Timer2) }
}

pub fn timer_clear_interrupt() {
    unsafe {
        TTC.clear_interrupt(Timer::Timer1);
    }
}

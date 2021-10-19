#![allow(dead_code)]

use core::ptr;

// TTCレジスタ
#[repr(C)]
pub struct Regs {
    pub clock_control: [u32; 3],
    pub counter_control: [u32; 3],
    pub counter_value: [u32; 3],
    pub interval_counter: [u32; 3],
    pub match_1_counter: [u32; 3],
    pub match_2_counter: [u32; 3],
    pub match_3_counter: [u32; 3],
    pub interrupt_register: [u32; 3],
    pub interrupt_enable: [u32; 3],
    pub event_control_timer: [u32; 3],
    pub event_register: [u32; 3],
}

pub enum Timer {
    Timer1 = 0,
    Timer2 = 1,
    Timer3 = 2,
}

use bitflags::bitflags;

bitflags! {
    pub struct ClockControl: u32 {
        const NONE = 0x00;
        const PRESCALER_ENABLE = 0x01;
        const CLOCK_SOURCE_EXTERNAL = 0x20;
        const EXTERNAL_CLOCK_EDGE_NEGATIVE  = 0x40;
    }
}

impl ClockControl {
    pub fn clear(&mut self) {
        self.bits = 0;
    }
}

bitflags! {
    pub struct CounterControl: u32 {
        const NONE = 0;
        const DISABLE = 0x01;
        const INTERVAL = 0x02;
        const DECREMENT = 0x04;
        const MATCH = 0x08;
        const RESET = 0x10;
        const OUTPUT_WAVEFORM_DISABLE = 0x20;
        const OUTPUT_WAVEFORM_POLARITY = 0x40;
    }
}

impl CounterControl {
    pub fn clear(&mut self) {
        self.bits = 0;
    }
}

bitflags! {
    pub struct Interrupt: u32 {
        const NONE = 0;
        const INTERVAL = 0x01;
        const MATCH1 = 0x02;
        const MATCH2 = 0x04;
        const MATCH3 = 0x08;
        const COUNTER_OVERFLOW = 0x10;
        const EVENT_TIMER_OVERFLOW = 0x20;
    }
}

impl Interrupt {
    pub fn clear(&mut self) {
        self.bits = 0;
    }
}

bitflags! {
    pub struct EventTimerControl: u32 {
        const NONE = 0x00;
        const ENABLE = 0x01;
        const LOW_LEVEL = 0x02;
        const CONTINUES_COUNTING_ON_OVERFLOW = 0x04;
        const TEST_MODE = 0x08;
    }
}

impl EventTimerControl {
    pub fn clear(&mut self) {
        self.bits = 0;
    }
}

pub struct Ttc {
    pub address: usize,
}

impl Ttc {
    pub const fn new(address: usize) -> Self {
        Ttc { address: address }
    }

    pub fn set_base_address(&mut self, address: usize) {
        self.address = address;
    }

    fn take(&self) -> &mut Regs {
        unsafe { &mut *(self.address as *mut Regs) }
    }

    pub fn reset(&self, timer: Timer) {
        let regs = self.take();
        let timer = timer as usize;
        unsafe {
            ptr::write_volatile(&mut regs.counter_control[timer], 0x31); // stop and reset
                                                                         //          ptr::write_volatile(&mut regs.counter_control[timer], 0x21);  // stop
            ptr::read_volatile(&mut regs.interrupt_register[timer]);
        }
    }

    pub fn set_clock_control(&self, timer: Timer, flags: ClockControl, prescale: u32) {
        assert_eq!(prescale & !0xf, 0);
        let regs = self.take();
        unsafe {
            ptr::write_volatile(
                &mut regs.clock_control[timer as usize],
                flags.bits() | (prescale << 1),
            );
        }
    }

    pub fn set_counter_control(&self, timer: Timer, flags: CounterControl) {
        let regs = self.take();
        unsafe {
            ptr::write_volatile(&mut regs.counter_control[timer as usize], flags.bits());
        }
    }

    pub fn get_counter_value(&self, timer: Timer) -> u32 {
        let regs = self.take();
        unsafe { ptr::read_volatile(&mut regs.counter_value[timer as usize]) }
    }

    pub fn set_interval_counter(&self, timer: Timer, value: u32) {
        let regs = self.take();
        unsafe {
            ptr::write_volatile(&mut regs.interval_counter[timer as usize], value);
        }
    }

    pub fn set_match1_counter(&self, timer: Timer, value: u32) {
        let regs = self.take();
        unsafe {
            ptr::write_volatile(&mut regs.match_1_counter[timer as usize], value);
        }
    }
    pub fn set_match2_counter(&self, timer: Timer, value: u32) {
        let regs = self.take();
        unsafe {
            ptr::write_volatile(&mut regs.match_2_counter[timer as usize], value);
        }
    }
    pub fn set_match3_counter(&self, timer: Timer, value: u32) {
        let regs = self.take();
        unsafe {
            ptr::write_volatile(&mut regs.match_3_counter[timer as usize], value);
        }
    }

    pub fn clear_interrupt(&self, timer: Timer) -> Option<Interrupt> {
        let regs = self.take();
        unsafe {
            let flags = ptr::read_volatile(&mut regs.interrupt_register[timer as usize]); // 読み出すとクリア
            Interrupt::from_bits(flags)
        }
    }

    pub fn enable_interrupt(&self, timer: Timer, flag: Interrupt) {
        let regs = self.take();
        unsafe {
            ptr::write_volatile(&mut regs.interrupt_enable[timer as usize], flag.bits());
            // Interrupt enable
        }
    }

    pub fn set_event_timer_control(&self, timer: Timer, flag: EventTimerControl) {
        let regs = self.take();
        unsafe {
            ptr::write_volatile(&mut regs.interrupt_enable[timer as usize], flag.bits());
            // Interrupt enable
        }
    }
}

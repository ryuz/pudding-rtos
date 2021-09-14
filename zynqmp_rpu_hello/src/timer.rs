

// TTCレジスタ
#[repr(C)]
struct Ttc {
    pub clock_control_1: u32,
    pub clock_control_2: u32,
    pub clock_control_3: u32,
    pub counter_control_1: u32,
    pub counter_control_2: u32,
    pub counter_control_3: u32,
    pub counter_value_1: u32,
    pub counter_value_2: u32,
    pub counter_value_3: u32,
    pub interval_counter_1: u32,
    pub interval_counter_2: u32,
    pub interval_counter_3: u32,
    pub match_1_counter_1: u32,
    pub match_1_counter_2: u32,
    pub match_1_counter_3: u32,
    pub match_2_counter_1: u32,
    pub match_2_counter_2: u32,
    pub match_2_counter_3: u32,
    pub match_3_counter_1: u32,
    pub match_3_counter_2: u32,
    pub match_3_counter_3: u32,
    pub interrupt_register_1: u32,
    pub interrupt_register_2: u32,
    pub interrupt_register_3: u32,
    pub interrupt_enable_1: u32,
    pub interrupt_enable_2: u32,
    pub interrupt_enable_3: u32,
    pub event_control_timer_1: u32,
    pub event_control_timer_2: u32,
    pub event_control_timer_3: u32,
    pub event_register_1: u32,
    pub event_register_2: u32,
    pub event_register_3: u32,
}

//const TIMER_INTNO: i32 = 74;


// OS用タイマ初期化ルーチン
#[no_mangle]
pub unsafe fn timer_initialize() {

    // TTC0 : 0xFF110000
    // TTC1 : 0xFF120000
    // TTC2 : 0xFF140000
    // TTC3 : 0xFF140000
    let ttc = &mut *(0xFF130000 as *mut Ttc);

    kernel::irc::interrupt_set_handler(74, Some(timer_int_handler));
    kernel::irc::interrupt_set_priority(74, 0xa0);
    kernel::irc::interrupt_enable(74);
//  kernel::irc::interrupt_disable(74);

    // タイマ動作開始
    core::ptr::write_volatile(&mut ttc.counter_control_1, 0x31); // stop and reset
    core::ptr::write_volatile(&mut ttc.counter_control_1, 0x21); // stop

    core::ptr::write_volatile(&mut ttc.clock_control_1, 0x03); // PS_VAL:1, PS_EN:1
    core::ptr::write_volatile(&mut ttc.interval_counter_1, 25000 - 1); // 1kHz (CPU_1x:100MHz)

    core::ptr::write_volatile(&mut ttc.interrupt_register_1, 0x01); // Interrupt : Interval
    core::ptr::write_volatile(&mut ttc.interrupt_enable_1, 0x01); // Interrupt enable

    core::ptr::write_volatile(&mut ttc.counter_control_1, 0x22); // start
}

pub fn timer_get_counter_value() -> u32 {
    unsafe {
        let ttc = &mut *(0xFF130000 as *mut Ttc);
        core::ptr::read_volatile(&mut ttc.counter_value_1)
    }
}

static mut TIMER_COUNTER: u32 = 0;

// タイマ割込みハンドラ
fn timer_int_handler() {
    unsafe {
        let ttc = &mut *(0xFF130000 as *mut Ttc);
        
        //  割込み要因クリア
        core::ptr::read_volatile(&mut ttc.interrupt_register_1); // 読み出すとクリア
        
        TIMER_COUNTER = TIMER_COUNTER.wrapping_add(1);
        if TIMER_COUNTER % 1000 == 0 {
            println!("timer");
        }
    }
}


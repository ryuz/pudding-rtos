#![no_std]
#![no_main]
#![feature(asm)]

mod bootstrap;

#[macro_use]
mod uart;
use uart::*;
mod timer;

use core::panic::PanicInfo;

fn wait(n: i32) {
    let mut v: i32 = 0;
    for i in 1..n {
        unsafe { core::ptr::write_volatile(&mut v, i) };
    }
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    println!("\r\n!!!panic!!!");
    loop {}
}


fn debug_print(str: &str)
{
    println!("{}", str);
}


mod memdump;

// use kernel::irc::pl390;




use jelly_kernel as kernel;
use kernel::*;

static mut STACK_INT: [isize; 512] = [0; 512];

static mut STACK0: [isize; 256] = [0; 256];
static mut STACK1: [isize; 256] = [0; 256];
static mut TASK0: Task = task_default!();
static mut TASK1: Task = task_default!();

// main
#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    wait(10000);
    println!("Hello world");
    
    /*
    println!("---- ICC ----");
    memdump::memdump(0xf9001000, 32);
    println!("---- IDC ----");
    memdump::memdump(0xf9000000, 32);
    println!("-------------");
    */

    println!("Start");
    {
        kernel::set_debug_print(Some(debug_print));

        kernel::initialize();
        kernel::interrupt::initialize(&mut STACK_INT);

        kernel::irc::pl390::initialize(0xf9001000, 0xf9000000);
        let pl390 = jelly_kernel::irc::pl390::take();

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


        timer::timer_initialize(timer_int_handler);
        
        
        wait(100);
//      println!("timer:{}", timer::timer_get_counter_value());
        
        TASK0.create(0, task0, 0, &mut STACK0);
        TASK1.create(1, task1, 1, &mut STACK1);
        TASK0.activate();
        TASK1.activate();

//        kernel::cpu::cpu_unlock();
    }
    println!("End");

    loop {
//        kernel::cpu::cpu_unlock();
        println!("timer:{} [s]", timer::timer_get_counter_value() as f32 / 100000000.0);
        wait(1000000);
    }
}



fn task0(_exinf:isize)
{
    println!("Task0");
}

fn task1(_exinf:isize)
{
    println!("Task1");
}


static mut TIMER_COUNTER: u32 = 0;

// タイマ割込みハンドラ
fn timer_int_handler() {
    unsafe {
        //  割込み要因クリア
        timer::timer_clear_interrupt();
        
        TIMER_COUNTER = TIMER_COUNTER.wrapping_add(1);
        if TIMER_COUNTER % 1000 == 0 {
            println!("timer irq");
//            TASK0.activate();
        }
    }
}


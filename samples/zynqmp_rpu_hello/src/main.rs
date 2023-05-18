#![no_std]
#![no_main]
#![allow(stable_features)]
//#![feature(asm)]

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

fn debug_print(str: &str) {
    println!("{}", str);
}

mod memdump;

use kernel::*;
use pudding_kernel as kernel;

static mut STACK_INT: [u8; 4096] = [0; 4096];

static mut STACKS: [[u8; 4096]; 5] = [[0; 4096]; 5];

static mut TASKS: [Task; 5] = [
    Task::new(),
    Task::new(),
    Task::new(),
    Task::new(),
    Task::new(),
];

static mut SEMS: [Semaphore; 5] = [
    Semaphore::new(1, Order::Fifo),
    Semaphore::new(1, Order::Fifo),
    Semaphore::new(1, Order::Fifo),
    Semaphore::new(1, Order::Fifo),
    Semaphore::new(1, Order::Fifo),
];

// main
#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    wait(10000);

    /*
    println!("---- ICC ----");
    memdump::memdump(0xf9001000, 32);
    println!("---- IDC ----");
    memdump::memdump(0xf9000000, 32);
    println!("-------------");
    */

    println!("\r\nkernel start");

    kernel::set_debug_print(Some(debug_print));

    kernel::initialize();
    kernel::interrupt::initialize(&mut STACK_INT);

    kernel::irc::pl390::initialize(0xf9001000, 0xf9000000);
    let pl390 = pudding_kernel::irc::pl390::take();

    let targetcpu: u8 = 0x01;
    pl390.icd_disable();

    // set TTC0-1
    pl390.icd_set_target(74, targetcpu);

    // PL
    for i in 0..8 {
        pl390.icd_set_target(121 + i, targetcpu);
        pl390.icd_set_config(121 + i, 0x01); // 0x01: level, 0x03: edge
    }
    for i in 0..8 {
        pl390.icd_set_target(136 + i, targetcpu);
        pl390.icd_set_config(136 + i, 0x01); // 0x01: level, 0x03: edge
    }

    pl390.icd_enable();

    timer::timer_initialize(timer_int_handler);

    for i in 0..5 {
        TASKS[i].create(i as isize, dining_philosopher, 1, &mut STACKS[i]);
        TASKS[i].activate();
    }

    kernel::idle_loop();
}

// 哲学者タスク
fn dining_philosopher(id: isize) {
    let id = id as usize;
    let left = id;
    let right = (id + 1) % 5;

    println!("[philosopher{}] dining start", id);

    loop {
        println!("[philosopher{}] thinking", id);
        kernel::sleep(rand_time());

        'dining: loop {
            unsafe {
                SEMS[left].wait();
            }
            {
                if unsafe { SEMS[right].polling().is_ok() } {
                    println!("[philosopher{}] eating", id);
                    kernel::sleep(rand_time());
                    unsafe {
                        SEMS[left].signal();
                    }
                    unsafe {
                        SEMS[right].signal();
                    }
                    break 'dining;
                } else {
                    unsafe {
                        SEMS[left].signal();
                    }
                }
            }
            println!("[philosopher{}] hungry", id);
            kernel::sleep(rand_time());
        }
    }
}

// 乱数
const RAND_MAX: u32 = 0xffff_ffff;
static mut RAND_SEED: u32 = 0x1234;
fn rand() -> u32 {
    unsafe {
        let x = RAND_SEED as u64;
        let x = ((69069 * x + 1) & RAND_MAX as u64) as u32;
        RAND_SEED = x;
        x
    }
}

fn rand_time() -> u32 {
    rand() % 1000 + 500
}

// タイマ割込みハンドラ
fn timer_int_handler() {
    //  割込み要因クリア
    timer::timer_clear_interrupt();

    // カーネルにタイムティック供給
    kernel::supply_time_tick(1);
}

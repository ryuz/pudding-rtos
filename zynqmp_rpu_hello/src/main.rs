#![no_std]
#![no_main]
#![feature(asm)]

mod bootstrap;

#[macro_use]
mod uart;
use uart::*;

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

use kernel::context;

static mut STACK0: [isize; 256] = [0; 256];
static mut STACK1: [isize; 256] = [0; 256];

static mut TASK0: context::ContextControlBlock = context::ContextControlBlock{sp:0};
static mut TASK1: context::ContextControlBlock = context::ContextControlBlock{sp:0};

// main
#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    wait(10000);
    println!("Hello! world!");

    TASK0 = context::ContextControlBlock::new(&mut STACK0, task0, 0);
    TASK1 = context::ContextControlBlock::new(&mut STACK1, task1, 1);
    TASK0.start();

    loop {}
}

extern "C" fn task0(_ext:isize)
{
    loop {
        wait(1000);
        println!("Task0");
        unsafe {
            TASK0.switch(&mut TASK1);
        }
    }
}

extern "C" fn task1(_ext:isize)
{
    loop {
        wait(1000);
        println!("Task1");
        unsafe {
            TASK1.switch(&mut TASK0);
        }
    }
}

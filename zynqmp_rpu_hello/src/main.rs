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


#[macro_use]
extern crate kernel;

use kernel::context::*;
use kernel::task::*;


// main
#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    wait(10000);
    println!("Hello! world!");
    
    println!("Start");
    {
        kernel::initialize();

        static mut STACK0: [isize; 256] = [0; 256];
        static mut STACK1: [isize; 256] = [0; 256];
        static mut TASK0: Task = task_default!();
        static mut TASK1: Task = task_default!();

        TASK0.create(0, task0, 0, &mut STACK0);
        TASK1.create(1, task1, 1, &mut STACK1);
        TASK0.activate();
        TASK1.activate();
    }
    println!("End");

    loop {}
}



fn task0(_exinf:isize)
{
    println!("Task0");
}

fn task1(_exinf:isize)
{
    println!("Task1");
}



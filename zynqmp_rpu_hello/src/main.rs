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


use kernel::task::*;

// main
#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    wait(10000);
    println!("Hello! world!");
    
    {
        let mut que: TaskQueue = TaskQueue::new();
        static mut STACK0: [isize; 256] = [0; 256];
        static mut STACK1: [isize; 256] = [0; 256];
        let mut task0: TaskControlBlock = TaskControlBlock::new(0, task0,0, &mut STACK0);
        let mut task1: TaskControlBlock = TaskControlBlock::new(0, task1,1, &mut STACK1);
        que.push_back(&mut task0);
        que.push_back(&mut task1);
        let t0 = que.pop_front();
        let t1 = que.pop_front();
        let t2 = que.pop_front();
        assert_eq!(t0.unwrap().get_priority(), 0);
        assert_eq!(t1.unwrap().get_priority(), 1);
        assert_eq!(t2.is_some(), false);
    }

    loop {}
}



fn task0(_ext:isize)
{
    loop {
        wait(1000);
        println!("Task0");
    }
}

fn task1(_ext:isize)
{
    loop {
        wait(1000);
        println!("Task1");
    }
}


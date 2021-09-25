#![no_std]
#![feature(asm)]


#[macro_use]
pub mod cpu;
pub use cpu::*;

#[macro_use]
pub mod context;

pub mod system;

pub mod irc;
pub mod interrupt;


#[macro_use]
pub mod task;
pub use task::*;

#[macro_use]
pub mod semaphore;
pub use semaphore::*;



pub unsafe fn initialize() {
    cpu::cpu_initialize();
    context::context_initialize();
}


static mut DEBUG_PRINT: Option<fn(str: &str)> = None;

pub fn set_debug_print(fnc: Option<fn(str: &str)>)
{
    unsafe {
        DEBUG_PRINT = fnc;
    }
}

pub fn debug_print(str: &str)
{
    unsafe {
        match DEBUG_PRINT {
            Some(print) => { print(str); },
            None => (),
        }
    }
}


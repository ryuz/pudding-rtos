#![no_std]
#![feature(asm)]


pub mod cpu;
pub mod irc;

pub mod system;

#[macro_use]
pub mod context;

#[macro_use]
pub mod task;

pub mod interrupt;


pub unsafe fn initialize() {
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

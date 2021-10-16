#![no_std]
#![feature(asm)]
#![feature(const_fn_trait_bound)]
#![feature(const_fn_fn_ptr_basics)]

pub type Priority = u32;
pub type RelTime = u32;
pub type ActCount = u32;
pub type SemCount = u32;

#[derive(Clone, Copy)]
pub enum Order {
    Priority,
    Fifo,
}

pub mod cpu;
pub use cpu::*;

pub mod context;
//pub use context::*;

pub mod system;

pub mod interrupt;
pub mod irc;

mod priority_queue;
mod timeout_queue;

pub mod task;
pub use task::*;

pub mod semaphore;
pub use semaphore::*;

pub mod time;
pub use time::*;

pub unsafe fn initialize() {
    cpu::cpu_initialize();
    context::context_initialize();
}

// 以下、デバッグ用の暫定

static mut DEBUG_PRINT: Option<fn(str: &str)> = None;

pub fn set_debug_print(fnc: Option<fn(str: &str)>) {
    unsafe {
        DEBUG_PRINT = fnc;
    }
}

pub fn debug_print(str: &str) {
    unsafe {
        match DEBUG_PRINT {
            Some(print) => {
                print(str);
            }
            None => (),
        }
    }
}

#![no_std]
#![feature(asm)]


pub mod system;
pub mod register;

#[macro_use]
pub mod context;

#[macro_use]
pub mod task;

pub mod cpu;
pub mod irc;


pub fn initialize() {
    context::context_initialize();
}


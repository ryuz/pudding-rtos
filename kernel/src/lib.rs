#![no_std]
#![feature(asm)]

#[cfg(target_arch = "arm")]
pub mod arm;
#[cfg(target_arch = "arm")]
pub use arm::*;

#[cfg(target_arch = "x86_64")]
pub mod x86_64;
#[cfg(target_arch = "x86_64")]
pub use x86_64::*;

//pub mod dummy;

pub mod context;
pub mod task;

pub fn initialize() {
    context::context_initialize();
}

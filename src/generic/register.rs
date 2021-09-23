#![allow(dead_code)]

use core::ptr;

// レジスタ書き込み
pub unsafe fn reg_write(addr: usize, data: usize) {
    let p = addr as *mut usize;
    ptr::write_volatile(p, data);
}

// レジスタ読み出し
pub unsafe fn reg_read(addr: usize) -> usize {
    let p = addr as *mut usize;
    ptr::read_volatile(p)
}

// レジスタ書き込み
pub unsafe fn reg8_write(addr: usize, data: u8) {
    let p = addr as *mut u8;
    ptr::write_volatile(p, data);
}

// レジスタ読み出し
pub unsafe fn reg8_read(addr: usize) -> u8 {
    let p = addr as *mut u8;
    ptr::read_volatile(p)
}

// レジスタ書き込み
pub unsafe fn reg16_write(addr: usize, data: u16) {
    let p = addr as *mut u16;
    ptr::write_volatile(p, data);
}

// レジスタ読み出し
pub unsafe fn reg16_read(addr: usize) -> u16 {
    let p = addr as *mut u16;
    ptr::read_volatile(p)
}

// レジスタ書き込み
pub unsafe fn reg32_write(addr: usize, data: u32) {
    let p = addr as *mut u32;
    ptr::write_volatile(p, data);
}

// レジスタ読み出し
pub unsafe fn reg32_read(addr: usize) -> u32 {
    let p = addr as *mut u32;
    ptr::read_volatile(p)
}

// レジスタ書き込み
pub unsafe fn reg64_write(addr: usize, data: u64) {
    let p = addr as *mut u64;
    ptr::write_volatile(p, data);
}

// レジスタ読み出し
pub unsafe fn reg64_read(addr: usize) -> u64 {
    let p = addr as *mut u64;
    ptr::read_volatile(p)
}

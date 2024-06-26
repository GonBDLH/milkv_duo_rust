use core::{
    arch::asm,
    ptr::{read_volatile, write_volatile},
};

const UART0_LSR: usize = 0x04140014;
const UART0_THR: usize = 0x04140000;

pub fn uart0_println(txt: &str) {
    for i in txt.chars() {
        uart0_send(i);
    }
    uart0_send('\r');
    uart0_send('\n');
}

pub fn uart0_send(char: char) {
    unsafe {
        while read_volatile(UART0_LSR as *const u32) & 0x20 == 0 {}

        write_volatile(UART0_THR as *mut u32, char as u32);
    }
}

pub fn uart0_recv() -> char {
    unsafe {
        while read_volatile(UART0_LSR as *const u32) == 0 {}

        read_volatile(UART0_THR as *mut u32) as u8 as char
    }
}


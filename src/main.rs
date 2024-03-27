#![no_std]
#![no_main]

use core::{ptr::{read_volatile, write_volatile}, arch::global_asm};

global_asm!(
	".section .text
	.global _start
_start:
	/* BL33 information */
	j real_start
	.balign 4
	.word 0x33334c42  /* b'BL33' */
	.word 0xdeadbeea  /* CKSUM */
	.word 0xdeadbeeb  /* SIZE */
	.quad 0x80200000  /* RUNADDR */
	.word 0xdeadbeec
	.balign 4
	j real_start
	.balign 4
	/* Information end */"
);

const UART0_LSR: usize = 0x04140014;
const UART0_THR: usize = 0x04140000;

#[no_mangle]
pub fn uart0_send(char: char) {
    unsafe {
        while read_volatile(UART0_LSR as *const u32) & 0x20 == 0 {}

		write_volatile(UART0_THR as *mut u32, char as u32);
    }
}

#[no_mangle]
pub fn real_start() -> ! {
    uart0_send('a');

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
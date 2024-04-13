#![no_std]
#![no_main]

mod serial;

use core::arch::{asm, global_asm};

use serial::*;

use riscv::asm::ecall;

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

#[no_mangle]
pub fn real_start() -> ! {
    uart0_println("Hola mundo");

    loop {
        let recv = uart0_recv();
        uart0_send(recv);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

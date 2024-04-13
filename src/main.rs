#![no_std]
#![no_main]

mod serial;

use core::arch::global_asm;

use riscv::asm::{wfi, nop};
use serial::*;

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
    // uart0_println("Hola mundo");
    let msg = "Hola mundo desde SBI\n\r\0".as_bytes();
    let mut index = 0;
    let mut char = msg[index];

    while char != 0x00 {
        sbi_console_putchar(char as i64);
        index += 1;
        char = msg[index];
    }


    // uart0_println("Debug");

    // loop {
    //     let recv = sbi_console_getchar();
    //     if let Ok(ch) = recv {
    //         sbi_console_putchar(ch)
    //     }
    //     wfi();
    // }

    loop {
		wfi();
	}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

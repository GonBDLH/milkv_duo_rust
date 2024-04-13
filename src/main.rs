#![no_std]
#![no_main]

mod serial;

use core::arch::global_asm;

use riscv::asm::{nop, wfi};
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
    let mut c = msg[index];

    while c != 0x00 {
        if let Err(error_code) = sbi_console_putchar(c as char) {
            match error_code {
                -1 => uart0_println("Failed"),
                -2 => uart0_println("Not supported"),
                -3 => uart0_println("Invalid parameters"),
                -4 => uart0_println("Denied or not allowed"),
                -5 => uart0_println("Invalid address"),
                -6 => uart0_println("Already available"),
                -7 => uart0_println("Already started"),
                -8 => uart0_println("Already stopped"),
                -9 => uart0_println("Shared memory not available"),
                _ => unreachable!(),
            }
        }
        index += 1;
        c = msg[index];
    }

    //	while c != 0x00 {
    //		sbi::legacy::console_putchar(c);
    //		index += 1;
    //        c = msg[index];
    //	}

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

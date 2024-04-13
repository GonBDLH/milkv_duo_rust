#![no_std]
#![no_main]

mod serial;

use core::arch::global_asm;

use riscv::asm::wfi;
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

	while c != 0 {
		sbi::legacy::console_putchar(c);

		index += 1;
		c = msg[index];
	}

    uart0_println("Debug");

    loop {
        let recv = sbi::legacy::console_getchar();
        if let Some(ch) = recv {
			sbi::legacy::console_putchar(ch)
        }
        wfi();
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

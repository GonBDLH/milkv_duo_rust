#![no_std]
#![no_main]

mod serial;

use core::arch::global_asm;

use serial::uart0_println;

use riscv::asm::wfi;

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
		wfi();
	}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

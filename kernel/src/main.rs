#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

// https://cs140e.sergio.bz/docs/BCM2837-ARM-Peripherals.pdf

// 1.2.3 ARM physical addresses:
// bus address is 0x7Ennnnnn, but physical address is 0x3Fnnnnnn

// 6.1 General Purpose I/O (GPIO), Register View
const GPFSEL2: *mut u32 = 0x3F20_0008 as *mut u32;
const GPSET1: *mut u32 = 0x3F20_001C as *mut u32;
const GPCLR1: *mut u32 = 0x3F20_0028 as *mut u32;

mod boot {
    use core::arch::global_asm;

    global_asm!(".section .text._start");
}

/// The kernel entrypoint.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        // pin 21 -> output
        core::ptr::write_volatile(GPFSEL2, 1<<3);

        loop {
            gpio_high(21);

            // sleep
            for _ in 0..500000 {
                asm!("nop");
            }

            gpio_clear(21);
        }
    }
}

/// Sets the nth bit on the GPIO Pin Output Clear 1 address.
///
/// Table 6-11 – GPIO Output Clear Register 1
fn gpio_clear(pin: u32) {
    unsafe {
        core::ptr::write_volatile(GPCLR1, 1<<pin);
    }
}

/// Sets the nth bit on the GPIO Pin Output Set 1 address.
///
/// Table 6-9 – GPIO Output Set Register 1
fn gpio_high(pin: u32) {
    unsafe {
        core::ptr::write_volatile(GPSET1, 1<<pin);
    }
}

/// A panic handler that halts the kernel indefinitely.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    // asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    let RCC_AHB1ENR = 0x40023800 | 0x30; // RCC Clock Enable for AHB1
    let GPIOC_BASE = 0x40020800; // GPIOC Base Addr

    let GPIOC_MODER = GPIOC_BASE | 0x00; // GPIOC MODER
    let GPIOC_ODR = GPIOC_BASE | 0x14; // GPIOC ODR
    let GPIOC_BSRR = GPIOC_BASE | 0x18; // GPIOC_BSRRR
    let PIN_NO = 13; // PC13


    let x = 32;
    let mut y = x + 25;
    unsafe {

        // RCC to AHB
        core::ptr::write_volatile(RCC_AHB1ENR as *mut u32, 1 << 2);
        // // Set Pin as GPIO
        core::ptr::write_volatile(GPIOC_MODER as *mut u32, 1 << (2 * PIN_NO));
        loop {
            y = y + x;
            // Set GPIO
            core::ptr::write_volatile(GPIOC_BSRR as *mut u32, 1 << PIN_NO);

            for _ in 0..50000 {
                asm::nop(); // Delay
            }

            // // Clear GPIO
            core::ptr::write_volatile(0x40020818 as *mut u32, 1 << (PIN_NO * 2 + 3));

            for _ in 0..50000 {
                asm::nop(); // Delay
            }
        }
    }
}


#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;

mod gpio_hal;
use gpio_hal::GPIO;



#[entry]
fn main() -> ! {

    let RCC_AHB1ENR = 0x4002_3800 | 0x30; // RCC Clock Enable for AHB1

    unsafe {
        // RCC to AHB to enable GPIOB and GPIOC
        core::ptr::write_volatile(RCC_AHB1ENR as *mut u32, 0b110);
    }
    // Set Pin Mode as GPIO
    GPIO::set_mode(&gpio_hal::GPIOX::GPIOC, 13, gpio_hal::GpioModr::GPIO);

    // Set Pin Mode as GPIO
    GPIO::set_mode(&gpio_hal::GPIOX::GPIOB, 0, gpio_hal::GpioModr::INPUT);

    let mut idr: u32 = 0;

    loop {

        // Toggle GPIO
        gpio_hal::GPIO::toggle(gpio_hal::GPIOX::GPIOC, 13);

        for _ in 0..50000 {
            asm::nop(); // Delay
        }

        // Read Input from GPIOB
        idr = gpio_hal::GPIO::read_input_reg(&gpio_hal::GPIOX::GPIOB);
    }
}

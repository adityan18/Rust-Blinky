#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;

struct GPIO;

enum GpioModr {
    INPUT,
    GPIO,
    AF,
    ANA,
}
struct GpioConfigAddr {
    base: u32,
    mode_r: u32,
    od_r: u32,
    id_r: u32,
    bsr_r: u32,
}
enum GPIOX {
    GPIOA,
    GPIOB,
    GPIOC,
}

impl GPIO {
    fn gpio_configs(gpiox: &GPIOX) -> GpioConfigAddr {
        let base_addr: u32;
        base_addr = match gpiox {
            GPIOX::GPIOA => 0x4002_0000,
            GPIOX::GPIOB => 0x4002_0400,
            GPIOX::GPIOC => 0x4002_0800,
            _ => panic!("Something is Wrong"),
        };

        let config = GpioConfigAddr {
            base: base_addr,
            mode_r: base_addr | 0x00,
            id_r: base_addr | 0x10,
            od_r: base_addr | 0x14,
            bsr_r: base_addr | 0x18,
        };

        config
    }

    pub fn set_mode(gpiox: &GPIOX, pin: u32, mode: GpioModr) {
        let config: GpioConfigAddr = Self::gpio_configs(gpiox);
        let mut current_mode: u32;
        unsafe {
            current_mode = core::ptr::read_volatile(config.mode_r as *mut u32);
        }

        let mut mask: u32 = 0b11;
        mask = mask << pin * 2;

        current_mode = current_mode & !(mask);
        current_mode = current_mode | ((mode as u32) << 2 * pin);

        unsafe {
            core::ptr::write_volatile(config.mode_r as *mut u32, current_mode);
        }
    }

    pub fn set_gpio(gpiox: &GPIOX, pin: u32) {
        let config: GpioConfigAddr = Self::gpio_configs(gpiox);
        let val: u32;
        val = 1 << pin;
        unsafe {
            core::ptr::write_volatile(config.bsr_r as *mut u32, val);
        }
    }

    pub fn clear_gpio(gpiox: &GPIOX, pin: u32) {
        let config: GpioConfigAddr = Self::gpio_configs(gpiox);
        let val: u32;
        val = 1 << (pin + 16);
        unsafe {
            core::ptr::write_volatile(config.bsr_r as *mut u32, val);
        }
    }

    pub fn toggle(gpiox: GPIOX, pin: u32) {
        let config: GpioConfigAddr = Self::gpio_configs(&gpiox);
        let current_state : u32;
        unsafe{
            current_state = core::ptr::read_volatile(config.od_r as *mut u32);
        }
        let val:u32 = (current_state >> pin) & 0b1;

        match val {
            0 => Self::set_gpio(&gpiox, pin),
            1 => Self::clear_gpio(&gpiox, pin),
            _ => panic!("Something Wrong")
        };
    }
}

#[entry]
fn main() -> ! {

    let RCC_AHB1ENR = 0x4002_3800 | 0x30; // RCC Clock Enable for AHB1

    unsafe {
        // RCC to AHB
        core::ptr::write_volatile(RCC_AHB1ENR as *mut u32, 1 << 2);
    }
    // // Set Pin Mode as GPIO
    GPIO::set_mode(&GPIOX::GPIOC, 13, GpioModr::GPIO);

    loop {

        // Toggle GPIO
        GPIO::toggle(GPIOX::GPIOC, 13);

        for _ in 0..50000 {
            asm::nop(); // Delay
        }
    }
}

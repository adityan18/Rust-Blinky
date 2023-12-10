use cortex_m_semihosting::nr::SEEK;

pub struct GPIO;

const GPIOA_BASE : u32 = 0x4002_0000;
const GPIOB_BASE : u32 = 0x4002_0400;
const GPIOC_BASE : u32 = 0x4002_0800;

pub enum GpioModr {
    INPUT,
    GPIO,
    AF,
    ANA,
}
pub struct GpioConfigAddr {
    base: u32,
    mode_r: u32,
    od_r: u32,
    id_r: u32,
    bsr_r: u32,
}
pub enum GPIOX {
    GPIOA,
    GPIOB,
    GPIOC,
}

impl GPIO {
    fn gpio_configs(gpiox: &GPIOX) -> GpioConfigAddr {
        let base_addr: u32;
        base_addr = match gpiox {
            GPIOX::GPIOA => GPIOA_BASE,
            GPIOX::GPIOB => GPIOB_BASE,
            GPIOX::GPIOC => GPIOC_BASE,
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
        let val: u32 = 1 << (pin + 16);
        unsafe {
            core::ptr::write_volatile(config.bsr_r as *mut u32, val);
        }
    }

    pub fn toggle(gpiox: GPIOX, pin: u32) {
        let current_state : u32 = Self::read_output_reg(&gpiox);
        let val:u32 = (current_state >> pin) & 0b1;

        match val {
            0 => Self::set_gpio(&gpiox, pin),
            1 => Self::clear_gpio(&gpiox, pin),
            _ => panic!("Something Wrong")
        };
    }

    pub fn read_output_reg(gpiox: &GPIOX) -> u32 {
        let config: GpioConfigAddr = Self::gpio_configs(&gpiox);
        let current_state : u32;
        unsafe{
            current_state = core::ptr::read_volatile(config.od_r as *mut u32);
        }
        current_state
    }

    pub fn read_input_reg(gpiox: &GPIOX) -> u32 {
        let config: GpioConfigAddr = Self::gpio_configs(&gpiox);
        let current_state : u32;
        unsafe{
            current_state = core::ptr::read_volatile(config.id_r as *mut u32);
        }
        current_state
    }
}
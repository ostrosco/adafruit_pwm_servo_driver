extern crate i2cdev;

use i2cdev::core::*;
use i2cdev::linux::LinuxI2CDevice;
use std::thread::sleep;
use std::time;

// Register defintions.
const PCA9685_ADDRESS: u16 = 0x40;
const MODE1: u8 = 0x00;
const MODE2: u8 = 0x01;
const SUBADR1: u8 = 0x02;
const SUBADR2: u8 = 0x03;
const SUBADR3: u8 = 0x04;
const PRESCALE: u8 = 0xFE;
const LED0_ON_L: u8 = 0x06;
const LED0_ON_H: u8 = 0x07;
const LED0_OFF_L: u8 = 0x08;
const LED0_OFF_H: u8 = 0x09;
const ALL_LED_ON_L: u8 = 0xFA;
const ALL_LED_ON_H: u8 = 0xFB;
const ALL_LED_OFF_L: u8 = 0xFC;
const ALL_LED_OFF_H: u8 = 0xFD;

// Bit defintions.
const RESTART: u8 = 0x80;
const SLEEP: u8 = 0x10;
const ALLCALL: u8 = 0x01;
const INVRT: u8 = 0x10;
const OUTDRV: u8 = 0x04;

// Other constants.
const SWRST: u8 = 0x06;
const PATH: &'static str = "/dev/i2c-0";

pub struct PCA9685 {
    device: LinuxI2CDevice,
}

impl PCA9685 {
    pub fn new() -> Self {
        let dev = LinuxI2CDevice::new(PATH, PCA9685_ADDRESS)
            .expect("Couldn't init I2C");
        PCA9685 { device: dev }
    }

    pub fn init(&mut self) -> () {
        self.device
            .smbus_write_byte_data(MODE2, OUTDRV)
            .unwrap();
        self.device
            .smbus_write_byte_data(MODE1, ALLCALL)
            .unwrap();
        sleep(time::Duration::from_millis(5));

        let mode1 = self.device.smbus_read_byte_data(MODE1).unwrap();
        let mode1 = mode1 & !SLEEP;
        self.device.smbus_write_byte_data(MODE1, mode1).unwrap();
        sleep(time::Duration::from_millis(5));
    }

    pub fn set_pwm(&mut self, channel: u8, on: u16, off: u16) -> () {
        self.device
            .smbus_write_byte_data(LED0_ON_L + 4 * channel, (on & 0xFF) as u8)
            .unwrap();
        self.device
            .smbus_write_byte_data(LED0_ON_H + 4 * channel, (on >> 8) as u8)
            .unwrap();
        self.device
            .smbus_write_byte_data(LED0_OFF_L + 4 * channel,
                                   (off & 0xFF) as u8)
            .unwrap();
        self.device
            .smbus_write_byte_data(LED0_OFF_H + 4 * channel, (off >> 8) as u8)
            .unwrap();
    }

    pub fn set_all_pwm(&mut self, on: u16, off: u16) -> () {
        self.device
            .smbus_write_byte_data(ALL_LED_ON_L, (on & 0xFF) as u8)
            .unwrap();
        self.device
            .smbus_write_byte_data(ALL_LED_ON_H, (on >> 8) as u8)
            .unwrap();
        self.device
            .smbus_write_byte_data(ALL_LED_OFF_L, (off & 0xFF) as u8)
            .unwrap();
        self.device
            .smbus_write_byte_data(ALL_LED_OFF_H, (off >> 8) as u8)
            .unwrap();
    }
}

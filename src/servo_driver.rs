extern crate i2cdev;

use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};
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

pub struct PCA9685 {
    device: LinuxI2CDevice,
}

impl PCA9685 {
    pub fn new(path: &str) -> Self {
        let dev = LinuxI2CDevice::new(path, PCA9685_ADDRESS)
            .expect("Couldn't init I2C");
        PCA9685 { device: dev }
    }

    pub fn init(&mut self) -> Result<(), LinuxI2CError> {
        try!(self.set_all_pwm(0, 0));
        try!(self.device.smbus_write_byte_data(MODE2, OUTDRV));
        try!(self.device.smbus_write_byte_data(MODE1, ALLCALL));
        sleep(time::Duration::from_millis(5));

        let mode1 = try!(self.device.smbus_read_byte_data(MODE1));
        let mode1 = mode1 & !SLEEP;
        try!(self.device.smbus_write_byte_data(MODE1, mode1));
        sleep(time::Duration::from_millis(5));
        Ok(())
    }

    pub fn set_pwm_freq(&mut self, freq_hz: f32) -> Result<(), LinuxI2CError> {
        let prescaleval = 25e6 / 4096.0 / freq_hz - 1.0;
        let prescale = (prescaleval + 0.5).floor() as u8;
        
        // Grab the old mode so we can revert our mode changes once we're done
        // setting the prescale.
        let old_mode = try!(self.device.smbus_read_byte_data(MODE1));

        // This new mode is used to put the device to sleep so we can set the
        // prescaler safely.
        let new_mode = old_mode & 0x7F | SLEEP;
        
        // Set the mode to sleep, set the prescaler, and wake up.
        try!(self.device.smbus_write_byte_data(MODE1, new_mode)); 
        try!(self.device.smbus_write_byte_data(PRESCALE, prescale));
        try!(self.device.smbus_write_byte_data(MODE1, old_mode));
        sleep(time::Duration::from_secs(5));
        
        // We need to turn on auto-incrememt for transmission to work properly.
        try!(self.device.smbus_write_byte_data(MODE1, old_mode | 0xA1));
        Ok(())
    }

    pub fn set_pwm(&mut self,
                   channel: u8,
                   on: u16,
                   off: u16)
                   -> Result<(), LinuxI2CError> {
        try!(self.device
                 .smbus_write_byte_data(LED0_ON_L + 4 * channel,
                                        (on & 0xFF) as u8));
        try!(self.device
                 .smbus_write_byte_data(LED0_ON_H + 4 * channel,
                                        (on >> 8) as u8));
        try!(self.device
                 .smbus_write_byte_data(LED0_OFF_L + 4 * channel,
                                        (off & 0xFF) as u8));
        try!(self.device
                 .smbus_write_byte_data(LED0_OFF_H + 4 * channel,
                                        (off >> 8) as u8));
        Ok(())
    }

    pub fn set_all_pwm(&mut self, on: u16, off: u16) -> Result<(), LinuxI2CError> {
        try!(self.device
                 .smbus_write_byte_data(ALL_LED_ON_L, (on & 0xFF) as u8));
        try!(self.device
                 .smbus_write_byte_data(ALL_LED_ON_H, (on >> 8) as u8));
        try!(self.device
                 .smbus_write_byte_data(ALL_LED_OFF_L, (off & 0xFF) as u8));
        try!(self.device
                 .smbus_write_byte_data(ALL_LED_OFF_H, (off >> 8) as u8));
        Ok(())
    }
}

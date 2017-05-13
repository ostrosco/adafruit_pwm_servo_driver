#![crate_name = "adafruit_pwm_servo_driver"]

extern crate i2cdev;

pub use self::servo_driver::PCA9685;
pub use i2cdev::linux::LinuxI2CError;
mod servo_driver;

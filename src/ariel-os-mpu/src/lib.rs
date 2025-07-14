#![no_std]

mod arch;

use arch::{Cpu, Mpu};

pub fn enable_mpu() {
    <Cpu as Mpu>::enable();
}

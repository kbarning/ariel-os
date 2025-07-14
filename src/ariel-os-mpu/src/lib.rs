#![no_std]

mod arch;

use arch::{Cpu, Mpu};

pub unsafe fn enable_mpu() {
    <Cpu as Mpu>::enable();
}

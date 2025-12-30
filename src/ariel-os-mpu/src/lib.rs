#![no_std]
#![expect(unsafe_code)]

mod arch;

use core::ops::RangeInclusive;

use arch::{Cpu, Mpu};

use crate::arch::MemoryAccess;

pub enum MpuRegionUsage {
    Flash = 0,
    Peripherals = 1,
    Stack = 2,
}

pub unsafe fn init_mpu() {
    <Cpu as Mpu>::init();
}

pub fn enable() {
    <Cpu as Mpu>::enable();
}

pub fn disable() {
    <Cpu as Mpu>::disable();
}

pub fn configure_stack(range: RangeInclusive<usize>) {
    <Cpu as Mpu>::configure_region(
        range,
        MpuRegionUsage::Stack as usize,
        MemoryAccess::READABLE | MemoryAccess::WRITEABLE,
    );
}

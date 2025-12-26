#![no_std]
#![expect(unsafe_code)]

mod arch;

use core::ops::RangeInclusive;

use arch::{Cpu, Mpu};

use crate::arch::MemoryAccess;

// FIXME reorder regions priority
pub enum MpuRegionUsage {
    Flash = 1,
    Peripherals = 2,
    Executable = 3,
    Stack = 4,
}

pub unsafe fn init_mpu() {
    <Cpu as Mpu>::init();
}

pub fn enable() {
    <Cpu as Mpu>::enable();
}

pub fn configure_stack(range: RangeInclusive<usize>) {
    // Disallow access, so that we detect a stack overflow with redzone
    <Cpu as Mpu>::configure_region(
        range,
        <Cpu as Mpu>::N_REGIONS - MpuRegionUsage::Stack as usize,
        MemoryAccess::READABLE | MemoryAccess::WRITEABLE,
    );
}

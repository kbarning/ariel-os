#![no_std]
#![expect(unsafe_code)]

mod arch;

use arch::{Cpu, Mpu};

use crate::arch::MemoryAccess;

// FIXME reorder regions priority
pub enum MpuRegionUsage {
    Flash = 1,
    Peripherals = 2,
    StackRedzone = 3,
}

pub unsafe fn init_mpu() {
    <Cpu as Mpu>::init();
}

pub fn context_switch(stack_begin: usize) {
    let truncated_start = stack_begin & !0b1_1111;

    const PAGESIZE: usize = 32;

    let redzone_range = if truncated_start == stack_begin {
        stack_begin..=stack_begin
    } else {
        stack_begin.saturating_add(PAGESIZE)..=stack_begin.saturating_add(PAGESIZE)
    };

    // Disallow access, so that we detect a stack overflow with redzone
    <Cpu as Mpu>::configure_region(
        redzone_range,
        <Cpu as Mpu>::N_REGIONS - MpuRegionUsage::StackRedzone as usize,
        MemoryAccess::empty(),
    );
}

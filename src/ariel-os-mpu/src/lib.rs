#![no_std]

mod arch;

use arch::{Cpu, Mpu};
use core::ops::Range;

use crate::arch::MemoryAccess;

// FIXME reorder regions priority
pub enum MpuRegionUsage {
    FLASH = 1,
    PERIPHERALS = 2,
    STACKREDZONE = 3,
}

pub unsafe fn init_mpu() {
    <Cpu as Mpu>::init();
}

pub fn context_switch(stack_addr: Range<usize>) {
    let truncated_start = stack_addr.start & !0b1_1111;

    const PAGESIZE: usize = 32;

    let redzone_range = if truncated_start == stack_addr.start {
        stack_addr.start..stack_addr.start.saturating_add(PAGESIZE)
    } else {
        stack_addr.start.saturating_add(PAGESIZE)..stack_addr.start.saturating_add(PAGESIZE * 2)
    };

    // Disallow access, so that we detect a stack overflow with redzone
    <Cpu as Mpu>::configure_region(
        redzone_range,
        <Cpu as Mpu>::N_REGIONS - MpuRegionUsage::STACKREDZONE as usize,
        MemoryAccess::empty(),
    );
}

#![no_std]

mod arch;

use arch::{Cpu, Mpu};
use ariel_os_debug::log::info;
use core::ops::Range;

use crate::arch::MemoryAccess;

// FIXME reorder regions priority
pub enum MpuRegionUsage {
    FLASH = 1,
    PERIPHERALS = 2,
    STACKREDZONE = 3,
}

pub unsafe fn init_mpu() {
    info!("Initializing MPU");
    <Cpu as Mpu>::init();
}

pub fn context_switch(stack_addr: Range<usize>) {
    let redzone_range = stack_addr.start..stack_addr.start.saturating_add(32); // FIXME correct?
    // Disallow access, so that we detect a stack overflow with redzone
    <Cpu as Mpu>::configure_region(
        redzone_range,
        <Cpu as Mpu>::N_REGIONS - MpuRegionUsage::STACKREDZONE as usize,
        MemoryAccess::empty(),
    );
}

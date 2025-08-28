#![no_std]

mod arch;

use arch::{Cpu, Mpu};
use ariel_os_debug::log::info;
use core::ops::Range;

use crate::arch::MemoryAccess;

pub enum MpuRegionUsage {
    FLASH = 1,
    PERIPHERALS = 2,
    OS_STACK = 3,
    THREAD_STACK = 4,
}

pub unsafe fn init_mpu() {
    info!("Initializing MPU");
    <Cpu as Mpu>::init();
}

pub fn context_switch(stack_addr: Range<usize>) {
    info!(
        "MPU switching context, configuring stack address {:x}-{:x}",
        stack_addr.start, stack_addr.end
    );

    <Cpu as Mpu>::configure_region(
        stack_addr,
        <Cpu as Mpu>::N_REGIONS - MpuRegionUsage::THREAD_STACK as usize,
        MemoryAccess::READABLE | MemoryAccess::WRITEABLE,
    );
}

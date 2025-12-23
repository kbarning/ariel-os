#![no_std]

mod arch;

use arch::{Cpu, Mpu};
use ariel_os_debug::log::info;
use core::ops::{Range, RangeInclusive};
use cortex_m::interrupt::enable;

use crate::arch::MemoryAccess;

// FIXME reorder regions priority
pub enum MpuRegionUsage {
    Flash = 1,
    Peripherals = 2,
    OS_Stack = 3,
    Thread_Stack = 4,
}

pub unsafe fn init_mpu() {
    <Cpu as Mpu>::init();
}

pub fn enable_mpu() {
    <Cpu as Mpu>::enable();
}

pub fn context_switch(stack_range: RangeInclusive<usize>) {
    let truncated_start = stack_range.start() & !0b1_1111;

    const PAGESIZE: usize = 32;

    // Round page up to 32-bit alignment to make sure we don't hit another process
    let redzone_range = if truncated_start == *stack_range.start() {
        *stack_range.start()..=stack_range.end().saturating_add(PAGESIZE)
    } else {
        stack_range.start().saturating_add(PAGESIZE)..=*stack_range.end()
    };

    <Cpu as Mpu>::configure_region(
        redzone_range,
        <Cpu as Mpu>::N_REGIONS - MpuRegionUsage::Thread_Stack as usize,
        MemoryAccess::empty(),
    );
}

#![no_std]

mod arch;

use arch::{Cpu, Mpu};
use ariel_os_debug::log::info;
use core::ops::Range;

pub unsafe fn init_mpu() {
    info!("Initializing MPU");
    <Cpu as Mpu>::init();
}

pub fn context_switch(exec_addr_range: Range<usize>, stack_addr: Range<usize>) {
    info!(
        "MPU switching context to executable memory {exec_addr_range:?} and stack address {stack_addr:?}"
    );
    <Cpu as Mpu>::context_switch(exec_addr_range, stack_addr);
}

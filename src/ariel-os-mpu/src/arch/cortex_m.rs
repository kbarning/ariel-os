use crate::Mpu;
use ariel_os_debug::println;
use core::{marker::PhantomData, ops::Range};
use cortex_m::{
    self as _, Peripherals,
    interrupt::{CriticalSection, disable},
    peripheral,
};
use cortex_mpu;

use crate::arch::MemoryAccess;

#[cfg(not(any(armv6m, armv7m, armv8m)))]
compile_error!("no supported ARM variant selected");

pub struct Cpu;

impl Mpu for Cpu {
    const N_REGIONS: usize = 8; // ARM v8m supports 8 regions

    fn init() {
        Self::disable();
        // Protect the program data and stack of the OS itself
        let flash_begin = 0x08000000; // FIXME hardcoded for stm32 at the moment
        let flash_end = flash_begin + 512 * 1024; // 512k length according to memory.x

        const MEMFAULTENA: u32 = 0b1 << 16;

        unsafe {
            let peripherals = Peripherals::steal();
            peripherals.SCB.shcsr.modify(|reg| reg | MEMFAULTENA); // Enable MEMFAULTENA so that the MEMFAULT handler will be called on MPU exception
        }
    }

    fn enable() {
        unsafe {
            let mpu = { &*cortex_m::peripheral::MPU::PTR };
            // The MPU should be enabled only in a critical section according to the Armv8-M Memory Model and Memory Protection manual
            critical_section::with(|_| {
                // We enable the MPU by setting the ENABLE bit in the ctrl register
                // We do not set the PRIVDEFENA flag, because ariel-os is always running in privileged mode
                // And we explicitly don't allow privileged code to use any kind of not configured
                // Memory. Also we don't set the HFNMIENA flag, so that the MPU is not active in a NMI handler
                const ENABLE: u32 = 0b1;
                mpu.ctrl.write(ENABLE); // Enable MPU
            });
        }
    }
    fn disable() {
        unsafe {
            let mpu = { &*cortex_m::peripheral::MPU::PTR };
            critical_section::with(|_| {
                mpu.ctrl.write(0x00); // Disable MPU
            });
        }
    }

    fn context_switch(exec_addr_range: Range<u32>, stack_addr: Range<u32>) {}

    fn protect_region(range: core::ops::Range<usize>, access: MemoryAccess) {}
    fn unprotect_region(range: core::ops::Range<usize>) {}
}

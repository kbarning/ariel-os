use crate::Mpu;
use cortex_m::{self as _, Peripherals, interrupt::CriticalSection, peripheral};
use cortex_mpu;

use crate::arch::MemoryAccess;

#[cfg(not(any(armv6m, armv7m, armv8m)))]
compile_error!("no supported ARM variant selected");

pub struct Cpu;

impl Mpu for Cpu {
    const N_REGIONS: usize = 8;

    fn enable() {
        unsafe {
            let mpu = { &*cortex_m::peripheral::MPU::PTR };
            let peripherals = Peripherals::steal();
            peripherals.SCB.shcsr.modify(|reg| reg | 0b1 << 16); // Enable MEMFAULTENA so that the MEMFAULT handler will be called on MPU exception
            critical_section::with(|_| {
                mpu.ctrl.write(0x05); // Enable MPU
            });
        }
    }
    fn disable() {}
    // TODO should this function return an unique object that can be used to unprotect/change a region later on?
    // Also should this keep track that the number of supported region is not exceeded?
    fn protect_region(range: core::ops::Range<usize>, access: MemoryAccess) {}
    fn unprotect_region(range: core::ops::Range<usize>) {}
}

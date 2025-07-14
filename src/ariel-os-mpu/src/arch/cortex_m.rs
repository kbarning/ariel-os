use crate::Mpu;
use cortex_m as _;
use cortex_mpu;

use crate::arch::MemoryAccess;

#[cfg(not(any(armv6m, armv7m, armv8m)))]
compile_error!("no supported ARM variant selected");

pub struct Cpu;

impl Mpu for Cpu {
    fn supported_regions() -> usize {
        let mut mpu = unsafe { Mpu::new(cortex_m::peripheral::MPU.) };
    }
    fn enable() {}
    fn disable() {}
    // TODO should this function return an unique object that can be used to unprotect/change a region later on?
    // Also should this keep track that the number of supported region is not exceeded?
    fn protect_region(range: core::ops::Range<usize>, access: MemoryAccess) {}
    fn unprotect_region(range: core::ops::Range<usize>) {}
}

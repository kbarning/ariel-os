bitflags::bitflags! {
    pub struct MemoryAccess : u8 {
        const READABLE = 0b1 << 0;
        const WRITEABLE = 0b1 << 1;
        const EXECUTABLE = 0b1 << 2;
        const CACHEABLE = 0b1 << 3;
    }
}

pub trait Mpu {
    const N_REGIONS: usize; // Defines the number of regions that the MPU supports
    fn enable();
    fn disable();
    // TODO should this function return an unique object that can be used to unprotect/change a region later on?
    // Also should this keep track that the number of supported region is not exceeded?
    fn protect_region(range: core::ops::Range<usize>, access: MemoryAccess);
    fn unprotect_region(range: core::ops::Range<usize>);
}

cfg_if::cfg_if! {
    if #[cfg(context = "cortex-m")] {
        mod cortex_m;
        pub use cortex_m::Cpu;
    }
    else
    {
        compile_error!("Unsupported mpu");
    }
    // TODO handle other architectures
}

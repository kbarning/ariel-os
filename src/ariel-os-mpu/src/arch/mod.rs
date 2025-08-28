bitflags::bitflags! {
    pub struct MemoryAccess : u8 {
        const READABLE = 0b1 << 0;
        const WRITEABLE = 0b1 << 1;
        const EXECUTABLE = 0b1 << 2;
        const CACHEABLE = 0b1 << 3;  // TODO do we need one kind of caching? ARM cortex v8-m supports multiple
    }
}

pub trait Mpu {
    const N_REGIONS: usize; // Defines the number of regions that the MPU supports

    fn init();
    fn enable();
    fn disable();
    fn configure_region(range: core::ops::Range<usize>, region_n: usize, access: MemoryAccess);
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

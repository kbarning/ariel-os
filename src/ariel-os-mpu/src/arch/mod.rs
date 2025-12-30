bitflags::bitflags! {
    pub struct MemoryAccess : u8 {
        const READABLE = 0b1 << 0; // Region ist lesbar
        const WRITEABLE = 0b1 << 1; // Region ist beschreibbar
        const EXECUTABLE = 0b1 << 2; // Region ist ausführbar
    }
}

pub trait Mpu {
    const N_REGIONS: usize; // Maximale Anzahl der unterstützen Regionen

    fn init();
    fn enable();
    fn disable();
    fn configure_region(
        range: core::ops::RangeInclusive<usize>,
        region_n: usize,
        access: MemoryAccess,
    );
}

cfg_if::cfg_if! {
    if #[cfg(all(any(armv8m)))] {
        mod cortex_m;
        pub use cortex_m::Cpu;
    }
    else
    {
        compile_error!("Unsupported mpu");
    }
}

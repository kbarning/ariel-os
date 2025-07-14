#![no_std]

bitflags::bitflags! {
    struct MemoryAccess : u8 {
        const Readable = 0b1 << 0;
        const Writeable = 0b1 << 1;
        const Executable = 0b1 << 2;
        const Cacheable = 0b1 << 3;
    }
}

trait MPU {
    // Configures and setups MPU. TODO possible startup parameters
    fn new() -> Self;
    fn supported_regions() -> usize; // TODO should this beeing fetched from the processor at runtime?
    fn enable();
    fn disable();
    // TODO should this function return an unique object that can be used to unprotect/change a region later on?
    // Also should this keep track that the number of supported region is not exceeded?
    fn protect_region(range: core::ops::Range<usize>, access: MemoryAccess);
    fn unprotect_region(range: core::ops::Range<usize>);
}

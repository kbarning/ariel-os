use crate::Mpu;
use ariel_os_debug::log::info;
use core::ops::Range;
use cortex_m::{self as _, Peripherals, peripheral::scb::SystemHandler};

use crate::arch::MemoryAccess;

#[cfg(not(any(armv6m, armv7m, armv8m)))]
compile_error!("no supported ARM variant selected");

pub struct Cpu;

impl Mpu for Cpu {
    const N_REGIONS: usize = 8; // ARM v8m supports 8 regions

    fn init() {
        // Protect the program data and stack of the OS itself

        const RAM_BEGIN: usize = 0x2000_0000;
        const RAM_END: usize = RAM_BEGIN + 256 * 1024;

        // Protect the stack data in ram
        Self::protect_region(
            RAM_BEGIN..RAM_END,
            Self::N_REGIONS - 2,
            MemoryAccess::READABLE | MemoryAccess::WRITEABLE,
        );

        const FLASH_BEGIN: usize = 0x800_0000; // FIXME hardcoded for stm32 at the moment
        const FLASH_END: usize = FLASH_BEGIN + 512 * 1024; // 512k length according to memory.x

        // Protect flash executable data
        // For safety, we assign the binary executable data the second highest region, because should some overlapping happen, the highest region of the stack should be preferred to prevent shell code execution
        Self::protect_region(
            FLASH_BEGIN..FLASH_END,
            Self::N_REGIONS - 1,
            MemoryAccess::EXECUTABLE | MemoryAccess::READABLE,
        );

        unsafe {
            const MEMFAULTENA: u32 = 0b1 << 16;
            let mut peripherals = Peripherals::steal();
            peripherals.SCB.shcsr.modify(|reg| reg | MEMFAULTENA); // Enable MEMFAULTENA so that the MEMFAULT handler will be called on MPU exception
            peripherals
                .SCB
                .set_priority(SystemHandler::MemoryManagement, 0xFE); // FIXEM higher priority then PendSv?
        }

        // Configuration done, enable MPU
        Self::enable();
    }

    fn enable() {
        critical_section::with(|_| {
            unsafe {
                // The MPU should be enabled only in a critical section according to the Armv8-M Memory Model and Memory Protection manual
                let mpu = { &*cortex_m::peripheral::MPU::PTR };
                // We enable the MPU by setting the ENABLE bit in the ctrl register
                // We do not set the PRIVDEFENA flag, because ariel-os is always running in privileged mode
                // And we explicitly don't allow privileged code to use any kind of not configured
                // Memory. Also we don't set the HFNMIENA flag, so that the MPU is not active in a NMI handler
                const ENABLE: u32 = 0b1;
                mpu.ctrl.write(ENABLE); // Enable MPU
            }
        });
    }
    fn disable() {
        critical_section::with(|_| {
            unsafe {
                let mpu = { &*cortex_m::peripheral::MPU::PTR };
                mpu.ctrl.write(0x00); // Disable MPU
            }
        });
    }

    fn context_switch(exec_addr_range: Range<usize>, stack_addr: Range<usize>) {}

    fn protect_region(range: core::ops::Range<usize>, region_n: usize, access: MemoryAccess) {
        info!("RAW PROTECT {:x}-{:x}", range.start, range.end);
        // Maybe be called from another critical section in sched(), but it is safe to do nested critical sections
        // It will be optimized to no-op
        critical_section::with(|_| {
            unsafe {
                let mpu = { &*cortex_m::peripheral::MPU::PTR };

                const OUTER_NON_CACHABLE: u32 = 0b0100 << 4;
                const INNER_NON_CACHABLE: u32 = 0b0100;

                // FIXME disable caching for now because of unwanted side effects
                mpu.mair[0].write(INNER_NON_CACHABLE | OUTER_NON_CACHABLE);

                // Select MPU region number
                mpu.rnr.write(region_n as u32);

                let executable = access.contains(MemoryAccess::EXECUTABLE);

                //[BASE=31:5|4:3=SH|AP=2:1|XN=0]
                let start_address_truncated = (range.start as u32) & !0b1_1111; // Only bit 31 to 5 are used for base address
                let shareability = 0b00u32 << 2;
                let access_permission = if access.contains(MemoryAccess::WRITEABLE) {
                    const READ_WRITE_PRIVILEGED: u32 = 0;
                    READ_WRITE_PRIVILEGED
                } else {
                    const READ_ONLY_PRIVILEGED: u32 = 0b10 << 1;
                    READ_ONLY_PRIVILEGED
                };
                let execution_permitted = (!executable) as u32;

                mpu.rbar.write(
                    start_address_truncated
                        | shareability
                        | access_permission
                        | execution_permitted,
                );

                // [LIMIT=31:5|4=PXN|ATTRIndx=3:1|EN=0]
                let end_address_truncated = (range.end as u32) & !0b1_1111; // Only bit 31 to 5 are used for limit address
                let privileged_execute_never = (!executable as u32) << 4;
                let attr_indx = 0b0u32 << 1; // FIXME preconfigure MAIR
                let enable = 0b1u32;

                mpu.rlar
                    .write(end_address_truncated | privileged_execute_never | attr_indx | enable);

                info!(
                    "Protecting {:x}-{:x}",
                    start_address_truncated, end_address_truncated
                );
            };
        });
    }
}

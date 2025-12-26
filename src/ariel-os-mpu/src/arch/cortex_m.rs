#![expect(unsafe_code)]

use crate::{Mpu, MpuRegionUsage};
use ariel_os_debug::log::info;
use cortex_m::{self as _, Peripherals};

use crate::arch::MemoryAccess;

#[cfg(not(any(armv6m, armv7m, armv8m)))]
compile_error!("no supported ARM variant selected");

pub struct Cpu;

impl Mpu for Cpu {
    const N_REGIONS: usize = 8; // ARM v8m supports 8 regions

    fn init() {
        critical_section::with(|_| {
            const FLASH_BEGIN: usize = 0x0800_0000; // FIXME hardcoded for stm32 at the moment
            const FLASH_END: usize = 0x0808_0000; // 512k length according to memory.x

            // Configure flash executable data
            // For safety, we assign the binary executable data the second highest region, because should some overlapping happen, the highest region of the stack should be preferred to prevent shell code execution
            Self::configure_region(
                FLASH_BEGIN..=FLASH_END,
                MpuRegionUsage::Flash as usize,
                MemoryAccess::EXECUTABLE | MemoryAccess::READABLE,
            );

            const PERIPHERALS_BEGIN: usize = 0x4000_0000; // FIXME hardcoded for stm32 at the moment
            const PERIPHERALS_END: usize = 0x4FFF_FFFF;

            // Configure peripherals memory
            Self::configure_region(
                PERIPHERALS_BEGIN..=PERIPHERALS_END,
                MpuRegionUsage::Peripherals as usize,
                MemoryAccess::WRITEABLE | MemoryAccess::READABLE,
            );

            unsafe {
                const MEMFAULTENA: u32 = 0b1 << 16;
                let mut peripherals = Peripherals::steal();
                // MemoryManagement has a higher priority then PendSV, because PendSV should always has the lowest
                peripherals.SCB.set_priority(
                    cortex_m::peripheral::scb::SystemHandler::MemoryManagement,
                    0xFE,
                );
                peripherals.SCB.shcsr.modify(|reg| reg | MEMFAULTENA); // Enable MEMFAULTENA so that the MEMFAULT handler will be called on MPU exception
            }
        });
    }

    fn enable() {
        unsafe {
            let mpu = { &*cortex_m::peripheral::MPU::PTR };
            // We enable the MPU by setting the ENABLE bit in the ctrl register
            // We don't set the PRIVDEFENA flag, so that accessing an un-configured region will be forbidden by the MPU
            // Also we don't set the HFNMIENA flag, so that the MPU is not active in a NMI handler
            const ENABLE: u32 = 0b1;
            mpu.ctrl.write(ENABLE); // Enable MPU
            cortex_m::asm::dsb(); // Recommended by Arm
            cortex_m::asm::isb();
        }
    }
    fn disable() {
        unsafe {
            let mpu = { &*cortex_m::peripheral::MPU::PTR };
            mpu.ctrl.write(0x00); // Disable MPU
        }
    }

    fn configure_region(
        range: core::ops::RangeInclusive<usize>,
        region_n: usize,
        access: MemoryAccess,
    ) {
        unsafe {
            let mpu = { &*cortex_m::peripheral::MPU::PTR };

            const OUTER_NON_CACHEABLE: u32 = 0b0100 << 4;
            const INNER_NON_CACHEABLE: u32 = 0b0100;

            // FIXME disable caching for now because of unwanted side effects
            mpu.mair[0].write(INNER_NON_CACHEABLE | OUTER_NON_CACHEABLE);

            // Select MPU region number
            mpu.rnr.write(region_n as u32);

            //[BASE=31:5|4:3=SH|AP=2:1|XN=0]
            let start_address_truncated = (*range.start() as u32) & !0b1_1111; // Only bit 31 to 5 are used for base address
            let shareability = 0b00u32 << 2; // Not used on single-core systems
            let access_permission = if access.contains(MemoryAccess::WRITEABLE) {
                const READ_WRITE_PRIVILEGED: u32 = 0;
                READ_WRITE_PRIVILEGED
            } else {
                const READ_ONLY_PRIVILEGED: u32 = 0b10 << 1;
                READ_ONLY_PRIVILEGED
            };
            // Ariel OS has no unprivileged code, so this bit is irrelevant
            let execute_never = 0b0;
            mpu.rbar
                .write(start_address_truncated | shareability | access_permission | execute_never);

            // [LIMIT=31:5|4=PXN|ATTRIndx=3:1|EN=0]
            let end_address_truncated = (*range.end() as u32) & !0b1_1111; // Only bit 31 to 5 are used for limit address
            info!(
                "REGION {:08x}-{:08x}",
                start_address_truncated,
                end_address_truncated | 0b1_1111,
            );
            let privileged_execute_never = (!access.contains(MemoryAccess::EXECUTABLE) as u32) << 4;
            let attr_indx = 0b0u32 << 1;
            let enable = 0b1u32;

            mpu.rlar
                .write(end_address_truncated | privileged_execute_never | attr_indx | enable);
        };
    }
}

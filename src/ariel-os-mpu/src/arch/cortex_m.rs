#![expect(unsafe_code)]

use crate::{Mpu, MpuRegionUsage};
use cortex_m::{self as _, Peripherals};

use crate::arch::MemoryAccess;

#[cfg(not(any(armv8m)))]
compile_error!("no supported ARM variant selected");

pub struct Cpu;

impl Mpu for Cpu {
    const N_REGIONS: usize = 8; // ARM v8m supports 8 regions

    fn init() {
        const FLASH_BEGIN: usize = 0x0800_0000;
        const FLASH_END: usize = 0x0807_FFFF;

        // Flash-Region konfigurieren
        // Diese ist ausführbar und lesbar, da der Prozessor von hier aus den Binärcode läd und ausführt
        Self::configure_region(
            FLASH_BEGIN..=FLASH_END,
            MpuRegionUsage::Flash as usize,
            MemoryAccess::EXECUTABLE | MemoryAccess::READABLE,
        );

        const PERIPHERALS_BEGIN: usize = 0x4000_0000;
        const PERIPHERALS_END: usize = 0x4FFF_FFFE;

        // Peripherie-Region konfigurieren
        // Diese Region ist les- und beschreibbar, damit Daten von Peripheriegeräten gelesen und geschrieben werden können
        Self::configure_region(
            PERIPHERALS_BEGIN..=PERIPHERALS_END,
            MpuRegionUsage::Peripherals as usize,
            MemoryAccess::WRITEABLE | MemoryAccess::READABLE,
        );

        unsafe {
            const MEMFAULTENA: u32 = 0b1 << 16;
            let mut peripherals = Peripherals::steal();
            // Der Handler MemoryManagement hat eine höhere Priorität als der Handler PendSV
            // PendSV benötigt die niedrigste Priorität im System.
            // So kann im Falle einer Zugriffsverletzung innerhalb von PendSV
            // MemoryManagement aufgerufen werden und so eine Fehlermeldung ausgeben
            peripherals.SCB.set_priority(
                cortex_m::peripheral::scb::SystemHandler::MemoryManagement,
                0xFE,
            );
            // Wenn eine Zugriffsverletzung ausgelöst wird, wird der
            // Handler MemoryManagement aufgerufen und kein Hardfault ausgelöst
            peripherals.SCB.shcsr.modify(|reg| reg | MEMFAULTENA);
        }
        // MPU einschalten
        Self::enable();
    }

    fn enable() {
        unsafe {
            let mpu = { &*cortex_m::peripheral::MPU::PTR };
            // Hier wir die MPU eingeschaltet. Des Weiteren wird das Flag PRIVDEFENA gesetzt.
            // Dadurch kann auf nicht konfigurierte Regionen zugegriffen werden, ohne dass es zu einer
            // Zugriffsverletzung kommt. Dies ist wichtig, damit auf globale Variablen zugegriffen werden kann.
            // Außerdem wird das Flag HFNMIENA nicht gesetzt. Dies führt dazu, dass die MPU
            // innerhab des Handler NMI nicht aktiv wird
            const ENABLE: u32 = 0b1;
            const PRIVDEFENA: u32 = 0b1 << 2;
            mpu.ctrl.write(ENABLE | PRIVDEFENA);
            // Arm empfiehlt das Auslösen einer Datensynchronisationsbarriere
            // sowie das aus Auslösen einer Anweisungsbarriere, damit die MPU erste
            // Eingeschaltet wird, wenn alle vorherigen Konfigurationen abgeschlossen sind
            cortex_m::asm::dsb();
            cortex_m::asm::isb();
        }
    }
    fn disable() {
        unsafe {
            let mpu = { &*cortex_m::peripheral::MPU::PTR };
            // MPU ausschalten
            mpu.ctrl.write(0x00);
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

            // Caching wird im Rahmen dieser Masterarbeit nicht verwendet
            mpu.mair[0].write(INNER_NON_CACHEABLE | OUTER_NON_CACHEABLE);

            // Zu konfigurierende Region auswählen
            mpu.rnr.write(region_n as u32);

            // Nur die oberen 27 Bits für die Adressierung werden verwendet.
            // Die unteren 5 Bits werden automatisch auf null gesetzt.
            //[BASE=31:5|4:3=SH|AP=2:1|XN=0]
            let start_address_truncated = (*range.start() as u32) & !0b1_1111;
            // Speicher wird nicht geteilt
            let shareability = 0b00u32 << 2;
            // In Armv8-M gibt es keine Möglichkeit, Lesezugriffe auf konfigurierten Regionen zu unterbinden
            // Eine Region kann nur als nur lesbar oder les- und beschreibbar konfiguriert werden
            let access_permission = if access.contains(MemoryAccess::WRITEABLE) {
                const READ_WRITE_PRIVILEGED: u32 = 0;
                READ_WRITE_PRIVILEGED
            } else {
                const READ_ONLY_PRIVILEGED: u32 = 0b10 << 1;
                READ_ONLY_PRIVILEGED
            };

            // Ausführbarkeit der Region festlegen
            let execute_never: u32 = !access.contains(MemoryAccess::EXECUTABLE) as u32;

            mpu.rbar
                .write(start_address_truncated | shareability | access_permission | execute_never);

            // Nur die oberen 27 Bits für die Adressierung werden verwendet.
            // Die unteren 5 Bits werden automatisch auf eins gesetzt.
            // [LIMIT=31:5|4=PXN|ATTRIndx=3:1|EN=0]
            let end_address_truncated = (*range.end() as u32) & !0b1_1111;

            // Da Ariel OS immer im privilegierten Modus ausgeführt wird, ist dieses Bit immer
            // nicht gesetzt. Die Ausführbarkeit einer Region entscheidet sich rein über das
            // Bit execute_never in dem Register rbar
            const PRIVILEGED_EXECUTE_NEVER: u32 = 0b0 << 4;
            // Always zero indexed, because we do not
            let attr_indx = 0b0u32 << 1;
            let enable = 0b1u32;

            // Das Überwachen der Region durch die MPU aktivieren
            mpu.rlar
                .write(end_address_truncated | PRIVILEGED_EXECUTE_NEVER | attr_indx | enable);
        };
    }
}

use ariel_os_debug::log::info;
use cortex_m::{self as _, Peripherals, peripheral::SCB};
use cortex_m_rt::{__RESET_VECTOR, ExceptionFrame, entry, exception};

use crate::stack::Stack;

// Table 2.5
// http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0553a/CHDBIBGJ.html
pub fn ipsr_isr_number_to_str(isr_number: usize) -> &'static str {
    match isr_number {
        0 => "Thread Mode",
        1 => "Reserved",
        2 => "NMI",
        3 => "HardFault",
        4 => "MemManage",
        5 => "BusFault",
        6 => "UsageFault",
        7..=10 => "Reserved",
        11 => "SVCall",
        12 => "Reserved for Debug",
        13 => "Reserved",
        14 => "PendSV",
        15 => "SysTick",
        16..=255 => "IRQn",
        _ => "(Unknown! Illegal value?)",
    }
}

/// Extra verbose Cortex-M HardFault handler
///
/// (copied from Tock OS)
///
/// # Safety
///
/// - must not be called manually
#[allow(non_snake_case)]
#[allow(unsafe_op_in_unsafe_fn)]
#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    use core::arch::asm;
    asm!("bkpt");

    let mode_str = "Kernel";

    let shcsr: u32 = core::ptr::read_volatile(0xE000ED24 as *const u32);
    let cfsr: u32 = core::ptr::read_volatile(0xE000ED28 as *const u32);
    let hfsr: u32 = core::ptr::read_volatile(0xE000ED2C as *const u32);
    let mmfar: u32 = core::ptr::read_volatile(0xE000ED34 as *const u32);
    let bfar: u32 = core::ptr::read_volatile(0xE000ED38 as *const u32);

    let iaccviol = (cfsr & 0x01) == 0x01;
    let daccviol = (cfsr & 0x02) == 0x02;
    let munstkerr = (cfsr & 0x08) == 0x08;
    let mstkerr = (cfsr & 0x10) == 0x10;
    let mlsperr = (cfsr & 0x20) == 0x20;
    let mmfarvalid = (cfsr & 0x80) == 0x80;

    let ibuserr = ((cfsr >> 8) & 0x01) == 0x01;
    let preciserr = ((cfsr >> 8) & 0x02) == 0x02;
    let impreciserr = ((cfsr >> 8) & 0x04) == 0x04;
    let unstkerr = ((cfsr >> 8) & 0x08) == 0x08;
    let stkerr = ((cfsr >> 8) & 0x10) == 0x10;
    let lsperr = ((cfsr >> 8) & 0x20) == 0x20;
    let bfarvalid = ((cfsr >> 8) & 0x80) == 0x80;

    let undefinstr = ((cfsr >> 16) & 0x01) == 0x01;
    let invstate = ((cfsr >> 16) & 0x02) == 0x02;
    let invpc = ((cfsr >> 16) & 0x04) == 0x04;
    let nocp = ((cfsr >> 16) & 0x08) == 0x08;
    let unaligned = ((cfsr >> 16) & 0x100) == 0x100;
    let divbysero = ((cfsr >> 16) & 0x200) == 0x200;

    let vecttbl = (hfsr & 0x02) == 0x02;
    let forced = (hfsr & 0x40000000) == 0x40000000;

    let xpsr = ef.xpsr();

    let ici_it = (((xpsr >> 25) & 0x3) << 6) | ((xpsr >> 10) & 0x3f);
    let thumb_bit = ((xpsr >> 24) & 0x1) == 1;
    let exception_number = (xpsr & 0x1ff) as usize;

    // Extract MMFSR bits (bits 0-7)
    let mmfsr = (cfsr & 0xFF) as u8;

    // Check if any Memory Management Fault bits are set (except MMARVALID itself)
    // We consider bits 0,1,3,4,5 as fault indicators.
    if mmfsr & ((1 << 0) | (1 << 1) | (1 << 3) | (1 << 4) | (1 << 5)) != 0 {
        mem_manage_fault_trace(mmfsr, cfsr);
    }

    panic!(
        "{} HardFault.\r\n\
         \tKernel version {}\r\n\
         \tr0  0x{:x}\r\n\
         \tr1  0x{:x}\r\n\
         \tr2  0x{:x}\r\n\
         \tr3  0x{:x}\r\n\
         \tr12 0x{:x}\r\n\
         \tlr  0x{:x}\r\n\
         \tpc  0x{:x}\r\n\
         \tprs 0x{:x} [ N {} Z {} C {} V {} Q {} GE {}{}{}{} ; ICI.IT {} T {} ; Exc {}-{} ]\r\n\
         \tsp  0x{:x}\r\n\
         \ttop of stack     0x{:x}\r\n\
         \tbottom of stack  0x{:x}\r\n\
         \tSHCSR 0x{:x}\r\n\
         \tCFSR  0x{:x}\r\n\
         \tHSFR  0x{:x}\r\n\
         \tInstruction Access Violation:       {}\r\n\
         \tData Access Violation:              {}\r\n\
         \tMemory Management Unstacking Fault: {}\r\n\
         \tMemory Management Stacking Fault:   {}\r\n\
         \tMemory Management Lazy FP Fault:    {}\r\n\
         \tInstruction Bus Error:              {}\r\n\
         \tPrecise Data Bus Error:             {}\r\n\
         \tImprecise Data Bus Error:           {}\r\n\
         \tBus Unstacking Fault:               {}\r\n\
         \tBus Stacking Fault:                 {}\r\n\
         \tBus Lazy FP Fault:                  {}\r\n\
         \tUndefined Instruction Usage Fault:  {}\r\n\
         \tInvalid State Usage Fault:          {}\r\n\
         \tInvalid PC Load Usage Fault:        {}\r\n\
         \tNo Coprocessor Usage Fault:         {}\r\n\
         \tUnaligned Access Usage Fault:       {}\r\n\
         \tDivide By Zero:                     {}\r\n\
         \tBus Fault on Vector Table Read:     {}\r\n\
         \tForced Hard Fault:                  {}\r\n\
         \tFaulting Memory Address: (valid: {}) {:#010X}\r\n\
         \tBus Fault Address:       (valid: {}) {:#010X}\r\n\
         ",
        mode_str,
        option_env!("RIOTCORE_KERNEL_VERSION").unwrap_or("unknown"),
        ef.r0(),
        ef.r1(),
        ef.r2(),
        ef.r3(),
        ef.r12(),
        ef.lr(),
        ef.pc(),
        xpsr,
        (xpsr >> 31) & 0x1,
        (xpsr >> 30) & 0x1,
        (xpsr >> 29) & 0x1,
        (xpsr >> 28) & 0x1,
        (xpsr >> 27) & 0x1,
        (xpsr >> 19) & 0x1,
        (xpsr >> 18) & 0x1,
        (xpsr >> 17) & 0x1,
        (xpsr >> 16) & 0x1,
        ici_it,
        thumb_bit,
        exception_number,
        ipsr_isr_number_to_str(exception_number),
        0u32,
        0u32,
        0u32,
        // faulting_stack as u32,
        // (_estack as *const ()) as u32,
        // (&_sstack as *const u32) as u32,
        shcsr,
        cfsr,
        hfsr,
        iaccviol,
        daccviol,
        munstkerr,
        mstkerr,
        mlsperr,
        ibuserr,
        preciserr,
        impreciserr,
        unstkerr,
        stkerr,
        lsperr,
        undefinstr,
        invstate,
        invpc,
        nocp,
        unaligned,
        divbysero,
        vecttbl,
        forced,
        mmfarvalid,
        mmfar,
        bfarvalid,
        bfar
    );
}

fn mem_manage_fault_trace(mmfsr: u8, cfsr: u32) {
    let mmfsr = (cfsr & 0xFF) as u8;

    info!(
        "Memory Management Fault Status Register (MMFSR): 0x{:02X}",
        mmfsr
    );

    if mmfsr & (1 << 0) != 0 {
        info!(" - Instruction Access Violation (IACCVIOL)");
    }
    if mmfsr & (1 << 1) != 0 {
        info!(" - Data Access Violation (DACCVIOL)");
    }
    if mmfsr & (1 << 3) != 0 {
        info!(" - MemManage Fault on Unstacking (MUNSTKERR)");
    }
    if mmfsr & (1 << 4) != 0 {
        info!(" - MemManage Fault on Stacking (MSTKERR)");
    }
    if mmfsr & (1 << 5) != 0 {
        info!(" - MemManage Fault on Lazy FP State Preservation (MLSPERR)");
    }

    if mmfsr & (1 << 7) != 0 {
        let mut peripherals = unsafe { Peripherals::steal() };
        let fault_addr = peripherals.SCB.mmfar.read();
        info!(" - Fault Address (MMFAR): 0x{:08X}", fault_addr);
    } else {
        info!(" - Fault Address (MMFAR) not valid");
    }
}

/// # Safety
///
/// - must not be called manually
#[exception]
unsafe fn DefaultHandler(_irqn: i16) {
    #[cfg(feature = "panic-printing")]
    ariel_os_debug::log::debug!("IRQn = {}", _irqn);

    ariel_os_debug::exit(ariel_os_debug::ExitCode::FAILURE);

    #[allow(clippy::empty_loop)]
    loop {}
}

#[entry]
fn main() -> ! {
    super::startup();
}

pub fn init() {
    // First, configure vector table address.
    // This is necessary when the vector table is not at its default position,
    // e.g., when there's a bootloader the default address.
    // Here, we're deriving the vector table address from the reset vector,
    // which is always the second entry in the vector table, after the initial
    // ISR stack pointer.

    unsafe {
        (*cortex_m::peripheral::SCB::PTR)
            .vtor
            .write(&__RESET_VECTOR as *const _ as u32 - 4)
    };

    #[cfg(any(armv7m_eabihf, armv8m_eabihf))]
    unsafe {
        let mut p = cortex_m::Peripherals::steal();
        // Enable fpu with full access
        p.SCB
            .set_fpu_access_mode(cortex_m::peripheral::scb::FpuAccessMode::Enabled);
        p.SCB.enable_fpu();
    }
}

/// Returns a `Stack` handle for the currently active thread.
pub(crate) fn stack() -> crate::stack::Stack {
    #[cfg(feature = "threading")]
    let (lowest, highest) = if cortex_m::register::control::read().spsel().is_psp() {
        // thread stack
        // Never panics when psp is active.
        ariel_os_threads::current_stack_limits().unwrap()
    } else {
        crate::isr_stack::limits()
    };

    // When threading is disabled, the isr stack is used.
    #[cfg(not(feature = "threading"))]
    let (lowest, highest) = crate::isr_stack::limits();

    Stack::new(lowest, highest)
}

/// Returns the current `SP` register value
pub(crate) fn sp() -> usize {
    let sp: usize;
    // Safety: reading SP is safe
    unsafe {
        core::arch::asm!(
            "mov {}, sp",
            out(reg) sp,
            options(nomem, nostack, preserves_flags)
        )
    };
    sp
}

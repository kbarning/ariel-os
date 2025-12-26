#![no_main]
#![no_std]
#![allow(unconditional_panic)]

use core::mem::MaybeUninit;

use ariel_os::debug::{ExitCode, exit, log::*};
use ariel_os::{mpu, thread::*};

#[ariel_os::thread(autostart)]
fn sandbox() {
    // Hier wird der C-Binärcode statisch mithilfe des Übersetzers platziert
    const C_BINARY: &'static [u8; 16] = include_bytes!("../../c_program/main.bin");
    let (stack_start, stack_end) = current_stack_limits().unwrap();
    mpu::configure_stack(stack_start..=stack_end);
    info!("MPU wird aktiviert");
    mpu::enable();

    unsafe {
        // Umwandeln des Array-Typens in ein Funktionszeiger
        let c_entry: extern "C" fn() = core::mem::transmute(C_BINARY.as_ptr());
        // Aufruf dieses Funktionszeigers
        c_entry();
    }
    mpu::disable();
    info!("Die C-Funktion wurde erfolgreich beendet");
    exit(ExitCode::SUCCESS);
}

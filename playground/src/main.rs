#![no_main]
#![no_std]
#![allow(unconditional_panic)]

use ariel_os::debug::{ExitCode, exit, log::info};
use ariel_os::thread::*;

#[ariel_os::thread(autostart)]
fn sandbox() {
    info!("Starting sandbox thread");
    // Hier wird der C-Binärcode statisch in dem Flash-Speicher mithilfe des Übersetzers platziert
    const C_BINARY: &[u8; 32] = include_bytes!("../../c_program/main.bin");
    unsafe {
        // Umwandeln des Array in ein Funktionszeiger
        let c_entry: extern "C" fn() = core::mem::transmute(C_BINARY.as_ptr());
        // Aufruf dieses Funktionszeigers
        c_entry();
    }
    info!("C-Binary returned successfully");
    exit(ExitCode::SUCCESS);
}

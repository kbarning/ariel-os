#![no_main]
#![no_std]
#![allow(unconditional_panic)]

use core::mem::MaybeUninit;

use ariel_os::debug::{ExitCode, exit, log::*};
use ariel_os::thread::*;

#[ariel_os::thread(autostart)]
fn sandbox() -> ! {
    let c_binary = include_bytes!("../../c_program/main.bin");
    info!("Entering C Function");

    unsafe {
        let c_entry: extern "C" fn() = core::mem::transmute(c_binary.as_ptr());
        c_entry();
    }

    info!("C function has returned");
    loop {}
}

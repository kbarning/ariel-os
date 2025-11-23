#![no_main]
#![no_std]
#![allow(unconditional_panic)]

use core::mem::MaybeUninit;

use ariel_os::debug::{ExitCode, exit, log::*};
use ariel_os::thread::*;

#[ariel_os::thread(autostart)]
fn sandbox() -> ! {
    let c_binary = include_bytes!("../../c_program/main.bin");
    unsafe {
        info!("Entering C Function: {}", c_binary.len());
        cortex_m::register::pc::write(c_binary.as_ptr() as u32);
    }
    loop {}
}

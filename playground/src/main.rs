#![no_main]
#![no_std]
#![allow(unconditional_panic)]

use core::mem::MaybeUninit;

use ariel_os::{
    debug::{ExitCode, exit, log::*},
    thread::current_stack_limits,
};

#[allow(unconditional_recursion)]
fn recursion() {
    // Ein Byte auf dem Stapelspeicher allokieren
    let arr: MaybeUninit<[u8; 1]> = MaybeUninit::uninit();
    // Verhindert, das diese Variable von dem Übersetzter wegoptimiert wird
    core::hint::black_box(arr);
    // Rekursiver Aufruf
    recursion();
}

#[ariel_os::thread(autostart)]
fn main() {
    info!("Stack start: {:x}", current_stack_limits().unwrap().0,);
    recursion();
}

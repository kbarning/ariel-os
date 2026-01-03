#![no_main]
#![no_std]
#![allow(unconditional_panic)]

use ariel_os::{
    debug::{ExitCode, exit, log::info},
    thread::current_stack_limits,
};

#[allow(unconditional_recursion)]
fn recursion() {
    // Ein Byte auf dem Stapelspeicher allokieren. Dies verhindert, dass der Übersetzter
    // Die Rekursion wegoptimiert (Tail-Call-Optimization)
    let arr: [u8; 1] = [0xff];
    // Verhindert, dass die Variable wegoptimiert wird
    core::hint::black_box(arr);
    // Rekursiver Aufruf
    recursion();
}

#[ariel_os::thread(autostart)]
fn a() {
    info!(
        "Thread a started, stack limit: 0x{:x}",
        current_stack_limits().unwrap().0
    );
    // Rekursion starten
    recursion();
    // Wird nie erreicht werden
    exit(ExitCode::SUCCESS);
}

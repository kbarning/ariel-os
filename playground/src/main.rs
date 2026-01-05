#![no_main]
#![no_std]
#![allow(unconditional_panic)]

use ariel_os::{
    debug::{ExitCode, exit, log::info},
    thread::current_stack_limits,
};

fn recursion() {
    // Ein 32 Bytes auf dem Stapelspeicher allokieren. Dies verhindert, dass der Übersetzter
    // die Rekursion wegoptimiert (Tail-Call-Optimization)
    let mut arr = [0u32; 32];
    // Verhindert, dass die Variable wegoptimiert wird
    core::hint::black_box(arr);
    // Zugriff auf ein Element in dem Array, welches hinter der 31 Bytes Grenze liegt.
    // Da der Stapelspeicher nach unten wächst, ist das nullte-Elemente hinter dieser Grenze
    arr[0] = 10;
    core::hint::black_box(arr[0]);
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

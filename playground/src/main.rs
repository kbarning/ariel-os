#![no_main]
#![no_std]
#![allow(unconditional_panic)]

use ariel_os::debug::{ExitCode, exit, log::*};
use ariel_os::thread::*;

#[ariel_os::thread(autostart)]
fn thread_a() {
    info!(
        "Thread A Running at address {:x} and sp {:x}",
        cortex_m::register::pc::read(),
        cortex_m::register::psp::read()
    );

    for _ in 0..1000 {
        info!("Thread A Looping 1");
    }

    yield_same();

    for _ in 0..10 {
        info!("Thread A Looping 2");
    }

    yield_same();
}

#[ariel_os::thread(autostart)]
fn thread_b() {
    info!(
        "Thread B Running at address {:x} and sp {:x}",
        cortex_m::register::pc::read(),
        cortex_m::register::psp::read()
    );

    for _ in 0..100 {
        info!("Thread B Looping 1");
    }

    yield_same();

    for _ in 0..10 {
        info!("Thread B Looping 2");
    }

    yield_same();
}

#[ariel_os::thread(autostart)]
fn thread_c() {
    info!(
        "Thread C Running at address {:x} and sp {:x}",
        cortex_m::register::pc::read(),
        cortex_m::register::psp::read()
    );

    for _ in 0..10 {
        info!("Thread C Looping 1");
    }

    for _ in 0..10 {
        info!("Thread C Looping 2");
    }

    yield_same();

    for _ in 0..10 {
        info!("Thread C Looping 3");
    }

    exit(ExitCode::SUCCESS);
}

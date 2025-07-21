#![no_main]
#![no_std]
#![allow(unconditional_panic)]

use ariel_os::debug::{log::*, println};
use ariel_os::thread::*;
use core::arch::asm;
use cortex_m::register::msp;

#[ariel_os::thread(autostart)]
fn thread_a() {
    info!(
        "Thread A Running at address {:x} and sp {}",
        cortex_m::register::pc::read(),
        cortex_m::register::psp::read()
    );

    for _ in 0..10 {
        info!("Thread A Looping");
    }

    yield_same();

    for _ in 0..10 {
        info!("Thread A Looping");
    }

    yield_same();

    loop {}
}

#[ariel_os::thread(autostart)]
fn thread_b() {
    info!(
        "Thread B Running at address {:x} and sp {}",
        cortex_m::register::pc::read(),
        cortex_m::register::psp::read()
    );

    for _ in 0..10 {
        info!("Thread B Looping");
    }

    yield_same();

    for _ in 0..10 {
        info!("Thread B Looping");
    }

    yield_same();
}

#[ariel_os::thread(autostart)]
fn thread_c() {
    info!(
        "Thread C Running at address {:x} and sp {}",
        cortex_m::register::pc::read(),
        cortex_m::register::psp::read()
    );

    for _ in 0..10 {
        info!("Thread C Looping");
    }

    for _ in 0..10 {
        info!("Thread C Looping again");
    }

    yield_same();

    for _ in 0..10 {
        info!("Thread C Looping");
    }

    yield_same();
}

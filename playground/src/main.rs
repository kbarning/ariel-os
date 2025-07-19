#![no_main]
#![no_std]
#![allow(unconditional_panic)]

use ariel_os::debug::{log::*, println};
use ariel_os::thread::*;
use cortex_m::register::msp;

#[ariel_os::thread(autostart)]
fn thread_a() {
    let stack_begin = msp::read();
    info!("Stack begin of tread a at: {:x}", stack_begin);
    info!("Thread A Running");

    for _ in 0..10 {
        info!("Thread A Looping");
    }

    yield_same();

    for _ in 0..10 {
        info!("Thread A Looping");
    }

    yield_same();
}

#[ariel_os::thread(autostart)]
fn thread_b() {
    info!("Thread B Running");

    for _ in 0..10 {
        info!("Thread B Looping");
    }

    yield_same();

    for _ in 0..10 {
        info!("Thread B Looping");
    }
}

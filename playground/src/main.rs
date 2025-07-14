#![no_main]
#![no_std]
#![allow(unconditional_panic)]

use ariel_os::debug::log::*;
use ariel_os::thread::*;

#[ariel_os::thread(autostart)]
fn thread_a() {
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

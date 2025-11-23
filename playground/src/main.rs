#![no_main]
#![no_std]
#![allow(unconditional_panic)]

use core::mem::MaybeUninit;

use ariel_os::debug::{ExitCode, exit, log::*};
use ariel_os::thread::*;

#[allow(unconditional_recursion)]
fn recursion() {
    let arr: MaybeUninit<[u8; 1]> = MaybeUninit::uninit();
    core::hint::black_box(arr);
    recursion();
}

#[ariel_os::thread(autostart)]
fn thread_a() {
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

    // unsafe {
    //     core::ptr::write(0x2000387f as *mut u8, 0x12);
    // }

    recursion();

    for _ in 0..100 {
        info!("Thread B Looping 1");
    }

    yield_same();

    for _ in 0..10 {
        info!("Thread B Looping 2");
    }

    exit(ExitCode::SUCCESS);
}

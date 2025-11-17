#![no_main]
#![no_std]
#![allow(unconditional_panic)]

use core::mem::MaybeUninit;

use ariel_os::debug::log::info;
use ariel_os::debug::{ExitCode, exit};
use ariel_os::thread::yield_same;

#[allow(unsafe_code)]
fn syscall(arg: u8) {
    // SAFETY: disable the warning
    unsafe {
        core::arch::asm!("svc {}", in(reg) arg);
    }
}

#[allow(unconditional_recursion)]
fn recursion() {
    let arr: MaybeUninit<[u8; 1]> = MaybeUninit::uninit();
    core::hint::black_box(arr);
    // Use to halt in debugger mode one step bevor overflow
    // let sp = cortex_m::register::psp::read();
    // if sp < 0x20003090 {
    //     cortex_m::asm::bkpt();
    // }
    recursion();
}

#[ariel_os::thread(autostart)]
fn thread_a() {
    // 20003030 -> 20003050

    recursion();

    for _ in 0..1000 {
        info!("Thread A Looping 1");
    }

    yield_same();

    for _ in 0..10 {
        info!("Thread A Looping 2");
    }

    yield_same();

    exit(ExitCode::SUCCESS);
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

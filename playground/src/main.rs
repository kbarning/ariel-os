#![no_main]
#![no_std]
#![allow(unconditional_panic)]

use ariel_os::debug::{ExitCode, exit};

#[ariel_os::thread(autostart)]
fn a() {
    exit(ExitCode::SUCCESS);
}

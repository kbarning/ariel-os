#![no_main]
#![no_std]
#![allow(unconditional_panic)]

use ariel_os::debug::{ExitCode, exit, log::*};

#[ariel_os::thread(autostart)]
fn main() {
    const MPU_TYPE_ADDR: *const u32 = 0xE000ED90 as *const u32;
    const MPU_CTRL_ADDR: *const u32 = 0xE000ED94 as *const u32;
    unsafe {
        let mpu_type = core::ptr::read(MPU_TYPE_ADDR);
        let mpu_ctrl = core::ptr::read(MPU_CTRL_ADDR);

        info!("MAX MPU REGS: {}", mpu_type >> 8);
        info!("MAX MPU CTRL: {mpu_ctrl:b}");
    }

    exit(ExitCode::SUCCESS);
}

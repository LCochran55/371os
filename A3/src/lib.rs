#![no_std] // Doesnt link to standard library.
#![no_main] // Disables all rust entry point.
#![feature(abi_x86_interrupt)]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(static_mut_refs)]

use core::panic::PanicInfo;
use x86_64::instructions::port::Port;

pub mod clock;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod serial;
pub mod vga;

pub const QEMU_PASS: u32 = 0x10 as u32;
pub const QEMU_FAIL: u32 = 0x11 as u32;

pub fn init() {
    interrupts::init_idt();
    gdt::init_gdt();
    //clock::init_clock();
}

pub fn exit_qemu(exit_code: u32) {
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code);
    }
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Binkle now running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QEMU_PASS);
}

pub fn test_panic_handler(info: &core::panic::PanicInfo) -> ! {
    serial_println!("[Binkle Fail :-C]");
    serial_println!("Error: {}", info);
    exit_qemu(QEMU_FAIL);
    hlt_loop();
}

// Entry point for test
#[cfg(test)]
#[unsafe(no_mangle)] // Doesnt mangle name of function
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    test_panic_handler(info)
}

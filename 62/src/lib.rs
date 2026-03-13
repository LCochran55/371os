#![no_std] // Doesnt link to standard library.
#![no_main] // Disables all rust entry point.
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod serial;
pub mod vga;

pub const QEMU_PASS: u32 = 0x10u32;
pub const QEMU_FAIL: u32 = 0x11u32;

pub fn exit_qemu(exit_code: u32) {
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code);
    }
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QEMU_PASS);
}

pub fn test_panic(info: &core::panic::PanicInfo) -> ! {
    serial_println!("[Fail]");
    serial_println!("Error: {}", info);
    exit_qemu(QEMU_FAIL);
    loop {}
}

// Entry point for test
#[cfg(test)]
#[unsafe(no_mangle)] // Doesnt mangle name of function
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    test_panic(info)
}


/*
#[test_case]
fn test_println() {
    println!("Binkle is the most epic cat boy");
}

#[test_case]
fn test_wrapping_println() {
    for _ in 0..534 {
        print!("Binkle");
    }
}

#[test_case]
fn bad() {
    assert!(false);
}

#[test_case]
fn good() {
    assert!(true);
}
*/


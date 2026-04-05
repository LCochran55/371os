#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(binkle_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use binkle_os::{println,print};

#[panic_handler]
fn test_panic(_info: &core::panic::PanicInfo) -> ! {
    binkle_os::serial_println!("[Pass]");
    binkle_os::exit_qemu(binkle_os::QEMU_PASS);
    loop {}
}

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
fn good() {
    assert!(true);
}


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();
    binkle_os::exit_qemu(binkle_os::QEMU_FAIL);
    loop {}
}





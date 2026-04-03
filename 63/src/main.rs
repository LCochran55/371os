#![no_std] // Doesnt link to standard library.
#![no_main] // Disables all rust entry point.
#![feature(custom_test_frameworks)]
#![test_runner(binkle_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

//mod vga;
use binkle_os::{println, serial_println};
use core::panic::PanicInfo;

#[unsafe(no_mangle)] // Doesnt mangle name of function
pub extern "C" fn _start() -> ! {
    println!("Hello Binkle{}", "!");

    binkle_os::init();

    #[cfg(test)]
    test_main();

    println!("Binkle did not crash!");
    binkle_os::hlt_loop();
}

// Called on a panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    binkle_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn test_panic(info: &core::panic::PanicInfo) -> ! {
    binkle_os::test_panic_handler(info)
}

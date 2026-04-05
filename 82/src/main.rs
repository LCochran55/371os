#![no_std] // Doesnt link to standard library.
#![no_main] // Disables all rust entry point.
#![feature(custom_test_frameworks)]
#![test_runner(binkle_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga;
use core::panic::PanicInfo;
use binkle_os::serial_println;

#[unsafe(no_mangle)] // Doesnt mangle name of function
pub extern "C" fn _start() -> ! {
    println!("Binkle World{}", "!");

    binkle_os::init();

    //unsafe {
      //  *(0xdeadbeef as *mut u8) = 42;
    //};

    #[cfg(test)]
    test_main();

    println!("Binkle did not crash! What a good driver.");
    loop {}
}


// Called on a panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn test_panic(info: &PanicInfo) -> ! {
    binkle_os::test_panic_handler(info)
}

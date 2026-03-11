#![no_std] // Doesnt link to standard library.
#![no_main] // Disables all rust entry point.
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod colors;
mod serial;
mod vga;

pub fn exit_qemu(exit_code: u32) {
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code);
    }
}


#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(0x10u32);  
}


#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    serial_println!("[Fail]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(0x11u32);
    loop{}
}

#[unsafe(no_mangle)] // Doesnt mangle name of function
pub extern "C" fn _start() -> ! {

    #[cfg(test)]
    test_main();

    println!("Binkle World{}", "!");

    loop {}
}

#[test_case]
fn test_println() {
    println!("Binkle is the most epic cat boy");
}

#[test_case]
fn test_wrapping_println() {
    for _ in 0..534{
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



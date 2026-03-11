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
fn test_runner(_tests: &[&dyn Fn()]) {
    let fs = [good, bad];
    for i in 0..fs.len() {
        serial_print!("Beginning test 0x{:02x}...", i);
        fs[i]();
        serial_println!("[Pass]");
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
    loop {}
}

#[unsafe(no_mangle)] // Doesnt mangle name of function
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();

    println!("I'm main{}", ".");
    loop {}
}

fn bad() {
    assert!(false);
}

fn good() {
    assert!(true);
}

#![no_std] // Doesnt link to standard library.
#![no_main] // Disables all rust entry point.
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

mod colors;
mod serial;
mod vga;


#[cfg(test)]
fn test_runner(_tests: &[&dyn Fn()]) {
    let fs = [_ex, _ex, _ex];
    for i in 0..fs.len() {
        serial_print!("Beginning test 0x{:02x}...", i);
        fs[i]();
        serial_println!("[Pass]");
    }
    unsafe { x86_64::instructions::port::Port::new(0xf4).write(0xAu32) };
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
    serial_println!("[Fail]");
    serial_println!("Failure: {}", info);
    unsafe { x86_64::instructions::port::Port::new(0xf4).write(0xFu32) };
    loop {}
}

#[unsafe(no_mangle)] // Doesnt mangle name of function
pub extern "C" fn _start() -> ! {
    println!("I'm main{}", ".");

    #[cfg(test)]
    test_runner(&[]);

    loop {}
}

fn _ex() {
    assert!(true);
    return;
}

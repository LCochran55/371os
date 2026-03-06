// main.rs

#![no_std] // Doesnt link to standard library.
#![no_main] // Disables all rust entry points.

mod colors;
mod vga;

//This function is called on panic.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[unsafe(no_mangle)] // Doesnt mangle name of function
pub extern "C" fn _start() -> ! {
    colors::colors();
    loop {}
}

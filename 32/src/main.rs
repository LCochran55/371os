// main.rs

#![no_std] // Doesnt link to standard library.
#![no_main] // Disables all rust entry points.


//This function is called on panic.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    loop{}
}

#[unsafe(no_mangle)] // Doesnt mangle name of function
pub extern "C" fn _start() -> ! {
    // Entry point function, linker looks for function named _start by default. 
    loop{}
}

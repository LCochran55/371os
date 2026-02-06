// main.rs

#![no_std]
#![no_main]
#![allow(unconditional_recursion)]


//This function is called on panic
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    panic(info)
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    _start()
}

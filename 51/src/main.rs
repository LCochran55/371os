// main.rs

#![no_std] // Doesnt link to standard library.
#![no_main] // Disables all rust entry points.

mod vga;


//This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("{}", _info);
    loop{}
}

#[unsafe(no_mangle)] // Doesnt mangle name of function
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    panic!("It is I, a panic!");
    //use core::fmt::Write;
    //let mut d = vga::Dummy { };
    //write!(d, "Hello {}!", "world");
    // Entry point function, linker looks for function named _start by default. 
    loop{}
}

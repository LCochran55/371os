// main.rs
#![no_std] // Doesnt link to standard library.
#![no_main] // Disables all rust entry points.


//This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)] // Doesnt mangle name of function
pub extern "C" fn _start() -> ! {
    let background: u16 = 0x10;
    let bright: u16 = 0x800;
    let blink: u16 = 0x8000;

    let rnbw: [u16; 6] = [
        0x0500 + bright,
        0x0400,
        0x0600 + bright,
        0x0200,
        0x0300,
        0x0100,
    ];

    // Makes a mutable raw pointer to 0xb8000
    let vga_buffer: *mut u16 = 0xb8000 as *mut u16;

    // Bytes we want to print
    let binkle_string = b"Binkle World!";

    let mut i = 0;
    for b in binkle_string {
        // NOTES:
        // Offset adds a signed offset to a pointer. -> adds a specified number of bytes to a new pointer
        // Buffer starts at 0xb8000, as i increases, adds i in bytes to buffer;
        // i = 0, VGA = 0xb8000 -> i = 1; VGA = 0xb8002
        //
        // Write overwrites a memory location with the given value without reading or dropping the old value.
        // Basically puts it there without doing anything to it or caring
        //
        // Then Get the bytes for our character and color using our bitwise operators
        //

        unsafe { vga_buffer.offset(i as isize).write(*b as u16 | rnbw[5]) };
        i += 1;
    }

    // Entry point function, linker looks for function named _start by default.
    loop {}
}

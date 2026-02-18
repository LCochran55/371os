fn main() {
    const BRIGHT: u16 = 0x800;
    const BLINK: u16 = 0x8000;

    const RNBW: [u16; 6] = [
        0x0500 + BRIGHT,
        0x0400,
        0x0600 + BRIGHT,
        0x0200,
        0x0300,
        0x0100,
    ];

    let binkle_string = b"Binkle World!";
    let mut i = 0;
    for b in binkle_string {
        println!("{:?} {:?}", i, b);
        i += 1;
    }

    let vga_buffer: *mut u16 = unsafe { 0xb8000 as *mut u16 };

    let mut i = 0;
    for b in binkle_string {
        println!("{:?}", i%6);
        println!("{:x}", RNBW[i%6]);
        i += 1
    }
}

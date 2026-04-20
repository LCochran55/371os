const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const BG: u16 = 0x10;
const BRIGHT: u16 = 0x800;
const BLINK: u16 = 0x8000;

//rep colors using an enum

const COLORS: [u16; 16] = [
    0x000,
    0x100,
    0x200,
    0x300,
    0x400,
    0x500,
    0x600,
    0x700,
    0x000 + BRIGHT,
    0x100 + BRIGHT,
    0x200 + BRIGHT,
    0x300 + BRIGHT,
    0x400 + BRIGHT,
    0x500 + BRIGHT,
    0x600 + BRIGHT,
    0x700 + BRIGHT,
];


pub fn colors() {
    let ch = 0x01u16;
    let vga_buffer: *mut u16 = 0xb8000 as *mut u16;

    for i in 0..BUFFER_HEIGHT {
        for j in 0..5 {
            let mut index = 8;
            for k in 0..16 {
                let mut color = (COLORS[k]) << 4 | (COLORS[index]);
                let offset = i * 80 + (j + 5 * k);

                unsafe { vga_buffer.offset(offset as isize).write(ch | color) };

                index += 1;
                if index == 16 {
                    index = 0
                };
            }
        }
    }
}

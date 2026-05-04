fn main() {
    #![no_std]
    #![no_main]

    use core::panic::PanicInfo;
    use core::ptr::write_volatile;

    const VGA_BUF: usize = 0xb8000;
    const WIDTH: usize = 80;
    const HEIGHT: usize = 25;
    const FOOD_CHAR: u8 = b'*';
    const FOOD_ATTR: u8 = 0x0E; // yellow on black

    // LCG parameters (modulus implicitly 2^32 via u32 overflow)
    // multiplier and increment chosen for decent period
    const LCG_A: u32 = 1664525;
    const LCG_C: u32 = 1013904223;

    static mut LCG_STATE: u32 = 0x1234_5678;

    #[inline]
    unsafe fn lcg_next() -> u32 {
        // state = state * a + c  (wraps modulo 2^32)
        LCG_STATE = LCG_STATE.wrapping_mul(LCG_A).wrapping_add(LCG_C);
        LCG_STATE
    }

    #[inline]
    unsafe fn rand_range(n: usize) -> usize {
        (lcg_next() as usize) % n
    }

    unsafe fn write_cell(x: usize, y: usize, ch: u8, attr: u8) {
        let off = (y * WIDTH + x) * 2;
        let ptr = (VGA_BUF + off) as *mut u8;
        write_volatile(ptr, ch);
        write_volatile(ptr.add(1), attr);
    }

    unsafe fn place_food_anywhere() {
        let x = rand_range(WIDTH);
        let y = rand_range(HEIGHT);
        write_cell(x, y, FOOD_CHAR, FOOD_ATTR);
    }

    #[no_mangle]
    pub extern "C" fn _start() -> ! {
        unsafe {
            place_food_anywhere();
            loop {
                core::arch::asm!("hlt");
            }
        }
    }

    #[panic_handler]
    fn panic(_info: &PanicInfo) -> ! {
        loop {}
    }
}

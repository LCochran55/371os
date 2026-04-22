use crate::print;
use core::fmt;
use core::fmt::Arguments;
use core::fmt::Write;
use spin::lazy::Lazy;

static mut LATEST: usize = 0;
static mut CLOCK_LATEST: usize = 70;
const MMIO: *mut u8 = 0xb8000 as *mut u8;
const COLOR: u8 = 0xF;

const ROWS: usize = 80;
const COLS: usize = 25;
const MAX: usize = ROWS * COLS;

fn char_to_vga(a: u8) {
    unsafe {
        let rel: *mut u8 = ((MMIO as usize) + (LATEST * 2)) as *mut u8;
        *rel = a;
        *((rel as usize + 1) as *mut u8) = COLOR;
        LATEST = LATEST + 1;
    }
}

fn scroll() {
    unsafe {
        for i in 80..MAX {
            let src: *mut u8 = ((MMIO as usize) + (i * 2)) as *mut u8;
            let dst: *mut u8 = ((MMIO as usize) + ((i - 80) * 2)) as *mut u8;
            *dst = *src;
            *((dst as usize + 1) as *mut u8) = COLOR;
        }
        for i in (MAX - 80)..MAX {
            let dst: *mut u8 = ((MMIO as usize) + ((i) * 2)) as *mut u8;
            *dst = 32;
            *((dst as usize + 1) as *mut u8) = COLOR;
        }
        LATEST = LATEST - 80;
    }
}

pub fn str_to_vga(s: &str) {
    let v = s.as_bytes();
    unsafe {
        for i in 0..v.len() {
            if LATEST > MAX {
                scroll();
            }
            match v[i] {
                10 => LATEST = ((LATEST / 80) + 1) * 80,
                _ => char_to_vga(v[i]),
            }
        }
    }
}

pub fn clock_str_to_vga(s: &str) {
    let v = s.as_bytes();
    unsafe {
        for i in 0..v.len() {
            let rel: *mut u8 = ((MMIO as usize) + CLOCK_LATEST * 2) as *mut u8;
            *rel = v[i];
            *((rel as usize + 1) as *mut u8) = COLOR;
            CLOCK_LATEST = CLOCK_LATEST + 1;
            if CLOCK_LATEST == 80 {
                CLOCK_LATEST = 70;
            }
        }
    }
}

pub struct Dummy {}

pub struct Clock {}

impl Write for Dummy {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        str_to_vga(s);
        Ok(())
    }
}

impl Write for Clock {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        clock_str_to_vga(s);
        Ok(())
    }
}

pub fn _print(args: Arguments) {
    let mut d = Dummy {};
    write!(d, "{}", args);
}

pub fn _clock_print(args: Arguments) {
    let mut c = Clock {};
    write!(c, "{}", args);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! clock_print {
    ($($arg:tt)*) => ($crate::vga::_clock_print(format_args!($($arg)*)));
}

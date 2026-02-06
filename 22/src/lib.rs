// Treat ourselves to a kb (1024 bits)
// 1024 >> 3 == 128 == 0x80
use std::mem::MaybeUninit;
use std::slice;
pub const SIZE: usize = 0x80;

// Not really a BUS but we gotta call it something.
static mut BUS: [u8; SIZE] = [0u8; SIZE];

// Return an index in BUS of s reserved bytes
pub fn malloc(s: usize) -> Option<usize> {
    let mut count = s;

    init();

    for row in 0..16 {
        let byte = unsafe { BUS[row] };

        for col in 0..8 {
            if (byte & (1 << col)) == 0 {
                count -= 1;
            } else {
                count = s;
            }

            if count == 0 {
                let current = (row * 8) + col;
                let first = current - s + 1;

                for i in first..current + 1 {
                    unsafe { BUS[i / 8] |= 1 << (i % 8) };
                }

                return Some(first);
            }
        }
    }
    return None;
}

// Zero the array except the mask.
fn init() {
    assert!(SIZE & (SIZE - 1) == 0);

    let mut bit_mask = SIZE >> 3;

    let mut bytes = 1;

    while bit_mask % 8 == 0 {
        bit_mask /= 8;
        bytes += 1;
    }

    for i in 0..bytes {
        unsafe { BUS[i] = 0xff };
    }
    return;
}

// Place val at loc
pub fn setter<T: std::fmt::Debug>(val: T, loc: usize) {
    let byte_slice: &[u8] =
        unsafe { slice::from_raw_parts((&raw const val).cast(), std::mem::size_of::<T>()) };
    unsafe { BUS[loc..(loc + byte_slice.len())].copy_from_slice(byte_slice) };
    return;
}

pub fn print_bus() {
    for i in 0..16 {
        for j in 0..8 {
            let val = unsafe { BUS[(i * 8) + j] };
            print!("{:x?} ", val);
        }
        println!();
    }
}

pub fn getter<T>(loc: usize) -> T {
    let slice = unsafe { &BUS[loc..(loc + std::mem::size_of::<T>())] };

    let mut val: MaybeUninit<T> = MaybeUninit::uninit();
    let ptr = val.as_mut_ptr();
    let mut slice2: &mut [u8] =
        unsafe { slice::from_raw_parts_mut(ptr.cast(), std::mem::size_of::<T>()) };
    slice2.copy_from_slice(slice);

    return unsafe { val.assume_init() };
}

#![no_std] // Doesnt link to standard library.
#![no_main] // Disables all rust entry point.
#![feature(custom_test_frameworks)]
#![test_runner(binkle_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(static_mut_refs)]

mod vga;

use binkle_os::{memory, serial_println};
use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use x86_64::VirtAddr;
use x86_64::structures::paging::{Page, Translate};

extern crate alloc;
use alloc::boxed::Box;

#[unsafe(no_mangle)] // Doesnt mangle name of function
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    println!("Binkle World{}", "!");

    binkle_os::init();

    //// ALLOC:
    let offset = x86_64::VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { binkle_os::memory::init(offset) };
    let mut frame_allocator =
        unsafe { binkle_os::memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };
    binkle_os::allocator::init_heap(&mut mapper, &mut frame_allocator).unwrap();

    //let offset = VirtAddr::new(boot_info.physical_memory_offset);
    //let mut mapper = unsafe { memory::init(offset) };
    //let mut frame_allocator = unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    //// map an unused page at address 0
    //let page = Page::containing_address(VirtAddr::new(0xdeadbeef000));
    //memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    //// convert the page to a raw pointer and write a value
    //// this writes the string `New!` to the screen through the new mapping
    //let ptr: *mut u64 = page.start_address().as_mut_ptr();
    // unsafe { ptr.write_volatile(0x_f021_f077_f065_f04e) };

    #[cfg(test)]
    test_main();

    //println!("Level 4 page table at: {:?}", x86_64::registers::control::Cr3::read().0.start_address());

    println!("Binkle did not crash! What a good OS.");
    binkle_os::hlt_loop();
}

// Called on a panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    binkle_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn test_panic(info: &PanicInfo) -> ! {
    binkle_os::test_panic_handler(info)
}

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(binkle_os::test_runner)]

#[panic_handler]
fn test_panic(info: &core::panic::PanicInfo) -> ! {
    binkle_os::serial_println!("{}", info);
    binkle_os::serial_println!("Binkle gives you a pass!");
    binkle_os::exit_qemu(binkle_os::QEMU_PASS);
    binkle_os::hlt_loop();
}

fn bad() {
    unsafe {
        *(0xdeadbeef as *mut u8) = 42;
    };
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    binkle_os::init();
    binkle_os::test_runner(&[&bad]);
    binkle_os::exit_qemu(binkle_os::QEMU_FAIL);
    binkle_os::hlt_loop();
}

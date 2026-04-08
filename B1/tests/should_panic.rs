#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(binkle_os::test_runner)]

#[panic_handler]
fn test_panic(_info: &core::panic::PanicInfo) -> ! {
    binkle_os::serial_println!("[Pass]");
    binkle_os::exit_qemu(binkle_os::QEMU_PASS);
    loop {}
}

fn bad() {
    assert!(false);
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    binkle_os::test_runner(&[&bad]);
    binkle_os::exit_qemu(binkle_os::QEMU_FAIL);
    loop {}
}

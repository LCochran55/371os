#![no_std]
#![no_main]
use spin::Lazy;
use core::panic::PanicInfo;
use binkle_os::serial_print;

use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

use blog_os::{exit_qemu, QemuExitCode, serial_println};
use x86_64::structures::idt::InterruptStackFrame;


static ref TEST_IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(blog_os::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
});

pub fn init_test_idt() {
    TEST_IDT.load();
}

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[ok]");
    exit_qemu(binkle_os::QEMU_PASS);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    serial_print!("stack_overflow::stack_overflow...\t");

    binkle_os::gdt::init_idt();
    init_test_idt();

    // trigger a stack overflow
    stack_overflow();

    panic!("Execution continued after stack overflow");
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow(); // for each recursion, the return address is pushed
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    binkle_os::test_panic_handler(info)
}

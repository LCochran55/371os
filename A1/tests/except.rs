#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    x86_64::instructions::interrupts::int3();
    binkle_os::hlt_loop();
}

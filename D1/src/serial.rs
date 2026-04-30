use core::fmt;
use uart_16550::SerialPort;

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_serial_print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}

pub fn _serial_print(args: fmt::Arguments) {
    use core::fmt::Write;
    let mut serial_port = unsafe { SerialPort::new(0x3F8) };
    serial_port.write_fmt(args).unwrap();
}

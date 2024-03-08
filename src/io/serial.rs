use conquer_once::spin::OnceCell;
use spin::Mutex;
use uart_16550::SerialPort;

/// The standard first serial port.
/// Use the [`serial_print`] and [`serial_println`]
/// macros to print to this.
pub static SERIAL1: OnceCell<Mutex<SerialPort>> = OnceCell::uninit();

pub fn init_serial1() -> Mutex<SerialPort> {
    let mut serial_port = unsafe { SerialPort::new(0x3F8) };
    serial_port.init();
    Mutex::new(serial_port)
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => ($crate::io::serial::_print(format_args!($($arg)*)));
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($($arg:tt)*) => ($crate::serial_print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;

    x86_64::instructions::interrupts::without_interrupts(|| {
        SERIAL1
            .get_or_init(init_serial1)
            .lock()
            .write_fmt(args)
            .expect("Printing to serial failed")
    });
}

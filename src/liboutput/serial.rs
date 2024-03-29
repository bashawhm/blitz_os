use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static!{
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = SerialPort::new(0x3f8);
        serial_port.init();
        Mutex::new(serial_port)
    };
}

pub fn print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        SERIAL1.lock().write_fmt(args).expect("Printing to serial port failed");
    });
}

// Prints to the host through the serial interface
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::liboutput::serial::print(format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! serial_println {
    () => (serial_print!("\n"));
    ($fmt:expr) => (serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (serial_print!(concat!($fmt, "\n"), $($arg)*));
}


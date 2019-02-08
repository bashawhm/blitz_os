#[macro_use]
pub mod vga_driver;
#[macro_use]
pub mod serial;

#[allow(unused_macros)]
#[macro_export]
macro_rules! println {
    () => {
        vga_println!();
        seral_println!();
    };
    ($fmt:expr) => {
        vga_println!($fmt);
        serial_println!($fmt);
    };
    ($fmt:expr, $($arg:tt)*) => {
        vga_println!(concat!($fmt), $($arg)*);
        serial_println!(concat!($fmt), $($arg)*);
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! print {
    () => {
        vga_print!();
        seral_print!();
    };
    ($fmt:expr) => {
        vga_print!($fmt);
        serial_print!($fmt);
    };
    ($fmt:expr, $($arg:tt)*) => {
        vga_print!(concat!($fmt), $($arg)*);
        // serial_print!(concat!($fmt), $($arg)*);
    };
}
use core::fmt::Write;

use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments)
{
    use x86_64::instructions::interrupts;
    
    interrupts::without_interrupts(||
    {
        SERIAL1.lock().write_fmt(args).expect("Unable perform serial printing!");
    });
}

/// Performs serial printing
#[macro_export]
macro_rules! sprint {
    ($($arg:tt)*) => {
        $crate::lumos::serial::_print(format_args!($($arg)*));
    };
}

/// Performs serial printing with a newline character at the end
#[macro_export]
macro_rules! sprintln {
    () => ($crate::sprint!("\n"));
    ($fmt:expr) => ($crate::sprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::sprint!(concat!($fmt, "\n"), $($arg)*));
}
use crate::lumos::buffer::vga::{WRITER, Color};
use x86_64::instructions::interrupts;

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments, fore: Color, back: Color) {
    use core::fmt::Write;

    interrupts::without_interrupts(||
    {
        WRITER.lock().set_colors(fore, back);
        WRITER.lock().write_fmt(args).unwrap();
    });
}

#[macro_export]
macro_rules! print
{
    ($fore:expr, $back:expr, $($arg:tt)*) =>
    ($crate::lumos::macros::_print(
        format_args!($($arg)*),
        $fore,
        $back
    ));
    
    ($fore:expr, $($arg:tt)*) =>
    ($crate::lumos::macros::_print(
        format_args!($($arg)*),
        $fore,
        crate::lumos::buffer::vga::Color::Black
    ));
    
    ($($arg:tt)*) => 
    ($crate::lumos::macros::_print(
        format_args!($($arg)*),
        crate::lumos::buffer::vga::Color::White,
        crate::lumos::buffer::vga::Color::Black
    ));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($fore:expr, $($arg:tt)*) => ($crate::print!($fore, Color::Black, "{}\n", format_args!($($arg)*)));
    ($($arg:tt)*) => ($crate::print!(Color::White, Color::Black, "{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => ($crate::print!(crate::lumos::buffer::vga::Color::Green, "{}", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! successln {
    ($($arg:tt)*) => ($crate::println!(crate::lumos::buffer::vga::Color::Green, "{}", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => ($crate::print!(crate::lumos::buffer::vga::Color::Yellow, "{}", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! warningln {
    ($($arg:tt)*) => ($crate::println!(crate::lumos::buffer::vga::Color::Yellow, "{}", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! fail {
    ($($arg:tt)*) => ($crate::print!(crate::lumos::buffer::vga::Color::Red, "{}", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! failln {
    ($($arg:tt)*) => ($crate::println!(crate::lumos::buffer::vga::Color::Red, "{}", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! critical {
    ($($arg:tt)*) => ($crate::print!(crate::lumos::buffer::vga::Color::Magenta, Color::Black, "{}", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! criticalln {
    ($($arg:tt)*) => ($crate::println!(crate::lumos::buffer::vga::Color::Magenta, "{}", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => ($crate::print!(crate::lumos::buffer::vga::Color::White,
                                     crate::lumos::buffer::vga::Color::Red, "{}", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! fatalln {
    ($($arg:tt)*) => ($crate::print!(crate::lumos::buffer::vga::Color::White,
                                       crate::lumos::buffer::vga::Color::Red, "{}\n", format_args!($($arg)*)));
}
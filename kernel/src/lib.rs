#![no_std]
#![feature(abi_x86_interrupt)]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use crate::lumos::handlers::{gdt, interrupts};
use core::panic::PanicInfo;

pub mod lumos;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    fatal!("\n[LumOS Error]\n>    {}", _info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> !
{
    sprintln!("\n[LumOS Error]\n>\t{}", _info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn()
{
    fn run(&self) -> () {
        sprint!("{}...\t", core::any::type_name::<T>());
        self();
        sprintln!("\t[SUCCEED]");
    }
}

pub fn test_runner(tests: &[&dyn Testable])
{
    sprint!("Running {} test(s):\n>>>\t", tests.len());
    for test in tests
    {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

pub fn init()
{
    gdt::init();
    interrupts::init();
    unsafe { interrupts::PICS.lock().initialize(); };
    x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(test)]
use bootloader_api::{entry_point, BootInfo};

#[cfg(test)]
entry_point!(test_entry);

/// Entry point for `cargo test`
#[cfg(test)]
fn test_entry(_boot_info: &'static mut BootInfo) -> ! {
    init();
    test_main();
    hlt_loop();
}
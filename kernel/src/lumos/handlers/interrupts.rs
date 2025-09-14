use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use pic8259::ChainedPics;
use spin::{self, Mutex};
use crate::lumos::buffer::vga::Color;
use crate::{failln, print, println, hlt_loop};
use crate::lumos::handlers::gdt;

use lazy_static::lazy_static;

// IDT SETUP

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_h);
        idt.page_fault.set_handler_fn(page_fault_h);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_h)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt[InterruprIndex::Timer.as_usize()].set_handler_fn(timer_h);
        idt[InterruprIndex::Keyboard.as_usize()].set_handler_fn(keyboard_h);

        idt
    };
}

pub fn init()
{
    IDT.load();
}

fn exception_msg(exp_type: &str, sf: &InterruptStackFrame)
{
    println!(Color::Red, "Thrown [{} EXCEPTION]:\n{:#?}", exp_type, sf);
}

// CPU interrupt exception funcions

extern "x86-interrupt" fn breakpoint_h(sf: InterruptStackFrame)
{
    exception_msg("BREAKPOINT", &sf);
}

extern  "x86-interrupt" fn page_fault_h(sf: InterruptStackFrame, err: PageFaultErrorCode)
{
    use x86_64::registers::control::Cr2;

    exception_msg("PAGE FAULT", &sf);
    failln!("Accessed address: {:?}", Cr2::read());
    failln!("Error: {:?}", err);

    hlt_loop();
}

extern "x86-interrupt" fn double_fault_h(sf: InterruptStackFrame, _err: u64) -> !
{
    panic!("[DOUBLE FAULT EXCEPTION]:\n{:#?}", &sf);
}

// CPU interrupt functions

extern "x86-interrupt" fn timer_h(_: InterruptStackFrame)
{
    unsafe
    {
        PICS.lock().notify_end_of_interrupt(InterruprIndex::Timer.as_u8());
    }
}

extern "x86-interrupt" fn keyboard_h(_: InterruptStackFrame)
{
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use x86_64::instructions::port::Port;
    lazy_static!
    {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
        Mutex::new(Keyboard::new(
            ScancodeSet1::new(),
            layouts::Us104Key,
            HandleControl::Ignore)
        );
    }

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);
    let sc: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(sc)
    {
        if let Some(key) = keyboard.process_keyevent(key_event)
        {
            match key {
                DecodedKey::Unicode(char) => print!(Color::White, Color::Black, "{}", char),
                DecodedKey::RawKey(_key) => (), // print!("{:?}", key),
            }
        }
    }

    unsafe
    {
        PICS.lock().notify_end_of_interrupt(InterruprIndex::Keyboard.as_u8());
    }
}
// PICs SETUP

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruprIndex
{
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruprIndex {
    fn as_u8(self) -> u8
    {
        self as u8
    }

    fn as_usize(self) -> usize
    {
        usize::from(self.as_u8())
    }
}
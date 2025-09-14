#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use lum_os_kernel::*;
use x86_64::structures::paging::Page;
use crate::lumos::buffer::vga::Color;
use bootloader_api::{config::Mapping, entry_point, BootInfo, BootloaderConfig};

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

entry_point!(entry, config = &BOOTLOADER_CONFIG);

pub mod lumos;

fn entry(boot_info: &'static mut BootInfo) -> !
{
    use lum_os_kernel::lumos::memory;
    use x86_64::{VirtAddr};

    print!("Initializing "); print!(Color::Blue, "LumOS"); println!("...");
    init();
    print!("Initialization ended with status:    ");
    println!(Color::Green, "[SUCCEED]");

    // let offset = match boot_info.physical_memory_offset.into_option() {
    //     Some(addr) => VirtAddr::new(addr),
    //     None => panic!("Physical memory offset not provided!")
    // };
// 
    // let mut mapper = unsafe {
    //     memory::init(offset)
    // };
// 
    // let mut fa = unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_regions) };

    // let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    // memory::create_mapping(page, &mut mapper, &mut fa);
    // 
    // let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    // unsafe {
    //     page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e);
    // };

    successln!("Uh! We are safe!");
    critical!("OR MAYBE NOT?!");

    println!("\n\n[[[ VARIOUS EXAMPLE MESSAGE SHOW ]]]\n");
    println!("Standard");
    successln!("Success");
    warningln!("Warning");
    failln!("Fail");
    criticalln!("Critical");
    fatal!("Fatal");

    hlt_loop();
}
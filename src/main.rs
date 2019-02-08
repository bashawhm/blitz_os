#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

#[macro_use]
mod liboutput;
mod interrupt;
pub mod gdt;
pub mod memory;

use core::panic::PanicInfo;
use bootloader::{bootinfo::BootInfo, entry_point};
use crate::memory::{create_example_mapping};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    halt();
}

pub fn halt() -> ! {
    x86_64::instructions::hlt();
    loop {}
}

entry_point!(kmain);

fn kmain(boot_info: &'static BootInfo) -> ! {
    println!("Blitz Online");

    gdt::init();
    interrupt::init_idt();
    unsafe { interrupt::PICS.lock().initialize(); }
    x86_64::instructions::interrupts::enable();

    let mut recursive_page_table = unsafe { 
        memory::init(boot_info.p4_table_addr as usize)
    };

    let mut frame_allocator = memory::init_frame_allocator(&boot_info.memory_map);

    println!("created allocator...");

    create_example_mapping(&mut recursive_page_table, &mut frame_allocator);
    unsafe { (0xdeadbeaf900 as *mut u64).write_volatile(0xf021f077f065f04e)};

    halt();
}
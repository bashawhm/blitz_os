#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

#[macro_use]
mod liboutput;
mod interrupt;
pub mod gdt;
pub mod memory;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    halt();
}

pub fn halt() -> ! {
    x86_64::instructions::hlt();
    loop {}
}

// static mut TIME: u64 = 0;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Blitz Online");

    gdt::init();
    interrupt::init_idt();
    unsafe { interrupt::PICS.lock().initialize(); }
    x86_64::instructions::interrupts::enable();

    use crate::memory::{create_example_mapping, EmptyFrameAllocator};
    let LEVEL_4_TABLE_ADDR: usize = 0o_177777_777_777_777_777_0000;
    let mut recursive_page_table = unsafe { memory::init(LEVEL_4_TABLE_ADDR) };

    create_example_mapping(&mut recursive_page_table, &mut EmptyFrameAllocator);
    unsafe { (0x1900 as *mut u64).write_volatile(0xf021f077f065f04e)};

    halt();
}
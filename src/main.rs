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

    /*use x86_64::structures::paging::PageTable;
    let level_4_table_ptr = 0xffff_ffff_ffff_f000 as *const PageTable;
    let level_4_table = unsafe { &*level_4_table_ptr };

    for i in 0..10 {
        println!("Entry {}: {:?}", i, level_4_table[i]);
    }*/

    use crate::memory::{translate_addr, self};
    let LEVEL_4_TABLE_ADDR: usize = 0o_177777_777_777_777_777_0000;
    let recursive_page_table = unsafe { memory::init(LEVEL_4_TABLE_ADDR) };
    //Identity mapped vga_buffer
    println!("0xb8000 -> {:?}", translate_addr(0xb8000, &recursive_page_table));
    // some code page
    println!("0x20010a -> {:?}", translate_addr(0x20010a, &recursive_page_table));
    // some stack page
    println!("0x57ac001ffe48 -> {:?}", translate_addr(0x57ac001ffe48, &recursive_page_table));

    halt();
}
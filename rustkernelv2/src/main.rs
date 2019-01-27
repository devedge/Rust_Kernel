// Entry point

// Override attributes first
#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

use core::panic::PanicInfo;
use rustkernelv2::{exit_qemu, print, println, serial_println};

// Entry point convention for Linux. Disable compiler name mangling.
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
  use rustkernelv2::interrupts::PICS;

  println!("Hello World{}", "!");

  // Set up the IDT to prevent a boot loop
  rustkernelv2::gdt::init();
  rustkernelv2::interrupts::init_idt();
  unsafe { PICS.lock().initialize() };
  x86_64::instructions::interrupts::enable();

  // try accessing & printing the page tables
  use x86_64::registers::control::Cr3;
  use x86_64::structures::paging::PageTable;

  let (level_4_page_table, _) = Cr3::read();
  println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

  let level_4_table_ptr = 0xffff_ffff_ffff_f000 as *const PageTable;
  let level_4_table = unsafe {&*level_4_table_ptr};
  for i in 0..10 {
    println!("Entry {}: {:?}", i, level_4_table[i]);
  }

  println!("It did not crash");
  // unsafe { exit_qemu(); }
  rustkernelv2::hlt_loop();
}

// Diverging function called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);
  rustkernelv2::hlt_loop();
}

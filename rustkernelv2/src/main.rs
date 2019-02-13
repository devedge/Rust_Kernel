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

  use rustkernelv2::memory::{self, translate_addr};

  const LEVEL_4_TABLE_ADDR: usize = 0o_177777_777_777_777_777_0000;
  let recursive_page_table = unsafe { memory::init(LEVEL_4_TABLE_ADDR) };

  // the identity-mapped vga buffer page
  println!("0xb8000 -> {:?}", translate_addr(0xb8000, &recursive_page_table));
  // some code page
  println!("0x20010a -> {:?}", translate_addr(0x20010a, &recursive_page_table));
  // some stack page
  println!("0x57ac001ffe48 -> {:?}", translate_addr(0x57ac001ffe48, &recursive_page_table));

  println!("It did not crash");
  rustkernelv2::hlt_loop();
}

// Diverging function called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);
  rustkernelv2::hlt_loop();
}

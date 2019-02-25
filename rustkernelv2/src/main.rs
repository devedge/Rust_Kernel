// Entry point

// Override attributes first
#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

use core::panic::PanicInfo;
use rustkernelv2::{println, memory};

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
  // end initializing GDT, IDT, PICS

  use rustkernelv2::memory::{create_example_mapping, EmptyFrameAllocator};

  const LEVEL_4_TABLE_ADDR: usize = 0o_177777_777_777_777_777_0000;
  let mut recursive_page_table = unsafe { memory::init(LEVEL_4_TABLE_ADDR) };

  create_example_mapping(&mut recursive_page_table, &mut EmptyFrameAllocator);
  unsafe { (0x1900 as *mut u64).write_volatile(0xf021f077f065f04e) };

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

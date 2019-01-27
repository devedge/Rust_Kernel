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

  // trigger a page fault
  let ptr = 0xdeadbeef as *mut u32;
  unsafe { *ptr = 42; }

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

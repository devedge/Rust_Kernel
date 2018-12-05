// Entry point

// Override attributes first
#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

use core::panic::PanicInfo;
use rustkernelv2::{exit_qemu, println, serial_println};

// Entry point convention for Linux. Disable compiler name mangling.
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
  println!("Hello World{}", "!");
  // serial_println!("Hello Host{}", "!");

  rustkernelv2::gdt::init();
  rustkernelv2::interrupts::init_idt();

  // trigger a page fault
  unsafe {
    *(0xdeadbeef as *mut u64) = 42;
  };

  println!("It did not crash");
  // unsafe { exit_qemu(); }

  loop {}
}

// Diverging function called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);
  loop {}
}

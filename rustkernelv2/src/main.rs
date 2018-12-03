// Entry point

// Override attributes first
#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

// Diverging function called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);
  loop {}
}

// Entry point convention for Linux. Disable compiler name mangling.
#[no_mangle]
pub extern "C" fn _start() -> ! {
  println!("Hello World{}", "!");

  loop {}
}

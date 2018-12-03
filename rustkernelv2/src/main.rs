// Entry point

// Override attributes first
#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

// Diverging function called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}

// Entry point convention for Linux. Disable compiler name mangling.
#[no_mangle]
pub extern "C" fn _start() -> ! {
  use core::fmt::Write;
  vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
  write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();

  loop {}
}

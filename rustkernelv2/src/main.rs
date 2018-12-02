// Entry point

// Override attributes first
#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Diverging function called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}

// Entry point convention for Linux. Disable compiler name mangling.
#[no_mangle]
pub extern "C" fn _start() -> ! {
  loop {}
}

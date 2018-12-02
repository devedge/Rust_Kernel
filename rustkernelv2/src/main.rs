// Entry point

// Override attributes first
#![no_std]

use core::panic::PanicInfo;

// Diverging function called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}

fn main() {}

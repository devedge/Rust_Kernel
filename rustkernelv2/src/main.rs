// Entry point

// Override attributes first
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

mod vga_buffer;
mod serial;

use core::panic::PanicInfo;

// Diverging function called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);
  loop {}
}

// Entry point convention for Linux. Disable compiler name mangling.
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
  println!("Hello World{}", "!");
  serial_println!("Hello Host{}", "!");

  unsafe { exit_qemu(); }

  loop {}
}

// Shut down qemu using the isa-debug-exit device (from qemu) located at
// x86's IO port 0xf4.
pub unsafe fn exit_qemu() {
  use x86_64::instructions::port::Port;

  let mut port = Port::<u32>::new(0xf4);
  port.write(0);
}

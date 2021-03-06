// Common libraries

// Override attributes
#![feature(abi_x86_interrupt)]
#![cfg_attr(not(test), no_std)]
#![feature(alloc_error_handler)]

// use the built-in alloc crate
extern crate alloc;

// define modules, and make them publicly available outside this file
pub mod allocator;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod serial;
pub mod vga_buffer;
pub mod task;

// Shut down qemu using the isa-debug-exit device (from qemu) located at
// x86's IO port 0xf4.
pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

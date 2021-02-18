// Entry point

// Override attributes first
#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

extern crate alloc;

use bootloader::{bootinfo::BootInfo, entry_point};
use core::panic::PanicInfo;
use rustkernelv2::println;
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};

entry_point!(kernel_main);

// Entry point convention for Linux. Disable compiler name mangling.
#[cfg(not(test))]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rustkernelv2::interrupts::PICS;
    use rustkernelv2::allocator;
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");

    // Set up the IDT to prevent a boot loop
    rustkernelv2::gdt::init();
    rustkernelv2::interrupts::init_idt();
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
    // end initializing GDT, IDT, PICS

    use rustkernelv2::memory::{self, BootInfoFrameAllocator};

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    // create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));

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

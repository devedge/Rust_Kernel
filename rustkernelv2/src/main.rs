// Entry point

// Override attributes first
#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

use bootloader::{bootinfo::BootInfo, entry_point};
use core::panic::PanicInfo;
use rustkernelv2::println;

entry_point!(kernel_main);

// Entry point convention for Linux. Disable compiler name mangling.
#[cfg(not(test))]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rustkernelv2::interrupts::PICS;

    println!("Hello World{}", "!");

    // Set up the IDT to prevent a boot loop
    rustkernelv2::gdt::init();
    rustkernelv2::interrupts::init_idt();
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
    // end initializing GDT, IDT, PICS

    use rustkernelv2::memory;
    use x86_64::{structures::paging::Page, VirtAddr};

    let mut mapper = unsafe { memory::init(boot_info.physical_memory_offset) };
    let mut frame_allocator = memory::EmptyFrameAllocator;

    // map a previously unmapped page
    let page = Page::containing_address(VirtAddr::new(0x1000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // Write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

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

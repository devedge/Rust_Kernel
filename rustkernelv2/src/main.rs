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
    use x86_64::{structures::paging::MapperAllSizes, VirtAddr};

    let mapper = unsafe { memory::init(boot_info.physical_memory_offset) };

    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x20010a,
        // some stack page
        0x57ac_001f_fe48,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt); // use new mapper
        println!("{:?} -> {:?}", virt, phys);
    }

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

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

    use rustkernelv2::memory::active_level_4_table;

    let l4_table = unsafe { active_level_4_table(boot_info.physical_memory_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);
        }
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

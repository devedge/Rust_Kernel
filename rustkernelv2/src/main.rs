// Entry point

// Override attributes first
#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

use bootloader::{bootinfo::BootInfo, entry_point};
use core::panic::PanicInfo;
use rustkernelv2::{println, memory};

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

  let mut recursive_page_table = unsafe {
    memory::init(boot_info.p4_table_addr as usize)
  };

  let mut frame_allocator = memory::init_frame_allocator(&boot_info.memory_map);

  rustkernelv2::memory::create_mapping(&mut recursive_page_table, &mut frame_allocator);
  unsafe { (0xdeadbeaf900 as *mut u64).write_volatile(0xf021f077f065f04e) };

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

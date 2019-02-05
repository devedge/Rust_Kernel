// Paging management

use x86_64::PhysAddr;
use x86_64::structures::paging::PageTable;

// Returns the physical address for a given virtual address, or 'None' if the
// virtual address is not mapped
pub fn translate_addr(addr: usize) -> Option<PhysAddr> {
    // Introduce variables for the recursive index & the sign extension bits.
    // These values should technically not be hardcoded
    let r = 0o777; // recursive index
    let sign = 0o177777 << 48; // sign extension

    // Retrieve the page table indices of the address that we want to
    // translate
    let l4_idx = (addr >> 39) & 0o777; // level 4 index
    let l3_idx = (addr >> 30) & 0o777; // level 3 index
    let l2_idx = (addr >> 21) & 0o777; // level 2 index
    let l1_idx = (addr >> 12) & 0o777; // level 1 index
    let page_offset = addr & 0o7777;

    // calculate the table addresses
    let level_4_table_addr = 
        sign | (r << 39) | (r << 30) | (r << 21) | (r <<12);
    let level_3_table_addr = 
        sign | (r << 39) | (r << 30) | (r << 21) | (l4_idx <<12);
    let level_2_table_addr = 
        sign | (r << 39) | (r << 30) | (l4_idx << 21) | (l3_idx <<12);
    let level_1_table_addr = 
        sign | (r << 39) | (l4_idx << 30) | (l3_idx << 21) | (l2_idx <<12);

    // check that level 4 entry is mapped
    let level_4_table = unsafe { &*(level_4_table_addr as *const PageTable) };
}

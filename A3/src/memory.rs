use x86_64::registers::control::Cr3;
use x86_64::structures::paging::{OffsetPageTable, PageTable};

pub unsafe fn init(offset: x86_64::VirtAddr) -> OffsetPageTable<'static> {
    unsafe {
        let l4 = active_level_4_table(offset);
        OffsetPageTable::new(l4, offset)
    }
}

pub unsafe fn active_level_4_table(offset: x86_64::VirtAddr) -> &'static mut PageTable {
    unsafe {
        let (frame, _) = Cr3::read();

        let phys = frame.start_address();
        let virt = offset + phys.as_u64();
        let ptr: *mut PageTable = virt.as_mut_ptr();

        return &mut *ptr;
    }
}

//TRANSLATE_ADDR: Translate a virtual to a physical address.
//Traverse the four-level page table
//Return the mapped frame.
pub unsafe fn translate_addr(
    addr: x86_64::VirtAddr,
    offset: x86_64::VirtAddr,
) -> Option<x86_64::PhysAddr> {
    unsafe {
        let (l4, _) = Cr3::read(); //Read level 4
        let mut frame = l4;
        let indices = [
            addr.p4_index(),
            addr.p3_index(),
            addr.p2_index(),
            addr.p1_index(),
        ]; //Break addresses
        //into radices
        for &index in &indices {
            //Descend 4->3->2->1
            let virt = offset + frame.start_address().as_u64();
            let ptr: *const PageTable = virt.as_ptr();
            frame = match &(&*ptr)[index].frame() {
                Ok(x) => *x,
                Err(_) => return None,
            };
        }
        return Some(frame.start_address() + u64::from(addr.page_offset())); // Compose the address
        // from offset and start
        // address and return
    }
}

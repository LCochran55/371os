use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::registers::control::Cr3;
use x86_64::{
    PhysAddr, VirtAddr,
    structures::paging::{
        FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PageTableFlags, PhysFrame,
        Size4KiB,
    },
};

//Init a new OffsetPageTable
pub unsafe fn init(offset: VirtAddr) -> OffsetPageTable<'static> {
    unsafe {
        let l4 = active_level_4_table(offset);
        OffsetPageTable::new(l4, offset)
    }
}

//Returns a mutable reference to the active level 4 table.
pub unsafe fn active_level_4_table(offset: VirtAddr) -> &'static mut PageTable {
    let (frame, _) = Cr3::read();

    let phys = frame.start_address();
    let virt = offset + phys.as_u64();
    let ptr: *mut PageTable = virt.as_mut_ptr();

    return unsafe { &mut *ptr };
}

//TRANSLATE_ADDR: Translate a virtual to a physical address.
//Traverse the four-level page table
//Return the mapped frame.
pub unsafe fn translate_addr(addr: VirtAddr, offset: VirtAddr) -> Option<PhysAddr> {
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
// Create a FrameAllocator from the passed memory map.
//
//To create new page tables, need to create a frame allocator
//we use the memory_map that is passed by the bootloader as part of the BootInfo struct
pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

/// Create a FrameAllocator from the passed memory map.
impl BootInfoFrameAllocator {
    //memory map is provided by the BIOS/UEFI firmware
    //can only be queried early in boot process, bootloader calls the functions for us
    pub unsafe fn init(memory_map: &'static bootloader::bootinfo::MemoryMap) -> Self {
        //initializes a BootInfoFrameAllocator with a given memory map
        BootInfoFrameAllocator {
            memory_map,
            //Next field initialized with 0
            //increased for every frame allocation -> avoids returning the same frame twic
            next: 0,
        }
    }

    /// Returns an iterator over the usable frames specified in the memory map.
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        // get usable regions from memory map
        let regions = self.memory_map.iter();
        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);
        // map each region to its address range
        let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());
        // transform to an iterator of frame start addresses
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        // create `PhysFrame` types from the start addresses
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        //Usable_frames get an iterator of usable frames from the memory map
        //Iterator::nth function to get the frame with index self.next
        let frame = self.usable_frames().nth(self.next);
        //Before returning frame, increase self.next by one so we return following frame on next call
        self.next += 1;
        frame
    }
}

//maps a given virtual page to 0xb8000, the physical frame of the VGA text buffer.
pub fn create_example_mapping(
    page: Page,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

    let map_to_result = unsafe {
        //NOT SAFE, ONLY FOR TESTING
        mapper.map_to(page, frame, flags, frame_allocator)
    };
    map_to_result.expect("map_to failed").flush();
}

// FrameAllocator that always returns `None`.
pub struct EmptyFrameAllocator;

//Trait vv responsible for allocating frames for new page tables if they are needed by map_to
//unsafe bc implementer must guarantee that allocator yields only unused frames.
//Or undefined behavior might occur. EX: when two virtual pages are mapped to the same physical frame
unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        None
    }
}

#[global_allocator]
static ALLOCATOR: Dummy = Dummy;

use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

//To create a proper allocator we need:
    //A virtual memory range for the heap region
    //A map from this region to physical frames
pub const HEAP_START: usize = 0x_C371_0000; // CS-371
pub const HEAP_SIZE: usize = 1 << 16;  // Arbitrary
                                       //
//initialize heap, using the Mapper previously implemented.
pub fn init_heap(
    mapper: &mut impl x86_64::structures::paging::Mapper<x86_64::structures::paging::Size4KiB>,
    frame_allocator: &mut impl x86_64::structures::paging::FrameAllocator<
        x86_64::structures::paging::Size4KiB,
    >,
) -> Option<()> {
    let page_range = {
        let heap_start = x86_64::VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = x86_64::structures::paging::Page::containing_address(heap_start);
        let heap_end_page = x86_64::structures::paging::Page::containing_address(heap_end);
        x86_64::structures::paging::Page::range_inclusive(heap_start_page, heap_end_page)
    };

    let flags = x86_64::structures::paging::PageTableFlags::PRESENT
        | x86_64::structures::paging::PageTableFlags::WRITABLE;
    for page in page_range {
        let frame = match frame_allocator.allocate_frame() {
            Some(f) => f,
            _ => return None,
        };
        unsafe {
            match mapper.map_to(page, frame, flags, frame_allocator) {
                Ok(m) => m.flush(),
                _ => return None,
            };
        }
    }

    return Some(());
}

pub struct Dummy;

unsafe impl GlobalAlloc for Dummy {
    //We return a null point on alloc and panic on dealloc.
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        panic!("dealloc should be never called Bad Binkle")
    }
}

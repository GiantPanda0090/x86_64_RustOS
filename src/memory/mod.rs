pub mod area_frame_allocator;
pub mod init;
pub mod paging;
pub mod heap_allocator;
pub use self::paging::remap_the_kernel;
pub use self::paging::test_paging;
use self::paging::PhysicalAddress;

//use multiboot2::{MemoryAreaIter, MemoryArea};

//pub use self::{AreaFrameAllocator};
//pub use self::{FrameAllocator};


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number: usize,

}

pub const PAGE_SIZE: usize = 4096;
//fram
impl Frame {

fn range_inclusive(start: Frame, end: Frame) -> FrameIter {
        FrameIter {
            start: start,
            end: end,
        }
    }

    fn containing_address(address: usize) -> Frame {
        Frame{ number: address / PAGE_SIZE }
    }

    //clone
    fn clone(&self) -> Frame {
       Frame { number: self.number }
    }

//export start address
fn start_address(&self) -> PhysicalAddress {
    self.number * PAGE_SIZE
}
}

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}

pub struct InactivePageTable {
    p4_frame: Frame,
}

impl InactivePageTable {
    pub fn new(frame: Frame) -> InactivePageTable {
        // TODO zero and recursive map the frame
        InactivePageTable { p4_frame: frame }
    }
}

struct FrameIter {
    start: Frame,
    end: Frame,
}

impl Iterator for FrameIter {
    type Item = Frame;

    fn next(&mut self) -> Option<Frame> {
        if self.start <= self.end {
            let frame = self.start.clone();
            self.start.number += 1;
            Some(frame)
        } else {
            None
        }
    }
 }


pub use self::area_frame_allocator::AreaFrameAllocator;
pub use self::heap_allocator::BumpAllocator;
pub use self::init::init;
//pub use self::area_frame_allocator::AreaFrameAllocator::{allocate_frame};

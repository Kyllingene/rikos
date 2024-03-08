use x86_64::structures::paging::{
    FrameAllocator, FrameDeallocator, Mapper, Page, PageTable, PageTableFlags, PhysFrame, Size4KiB,
};

use crate::BOOT_INFO;

fn remap_kernel(mapper: &mut impl Mapper<Size4KiB>, allocator: &mut impl FrameAllocator<Size4KiB>) {
    let boot_info = unsafe { BOOT_INFO.get() }.unwrap();

    let mut table = PageTable::new();
    
    table.zero();

    unsafe {
        mapper.map_to(
            Page::from(todo!()),
            PhysFrame::from(todo!()),
            PageTableFlags::WRITABLE,
            &mut TinyAllocator::new(allocator),
        );
    }

    table.zero();

    for section in boot_info.elf_sections().expect("must have elf sections") {
        // for frame in section.
    }
}

struct TinyAllocator([Option<PhysFrame>; 3]);

impl TinyAllocator {
    fn new(allocator: &mut impl FrameAllocator<Size4KiB>) -> Self {
        let mut f = || allocator.allocate_frame();
        let frames = [f(), f(), f()];
        TinyAllocator(frames)
    }
}

unsafe impl FrameAllocator<Size4KiB> for TinyAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        for frame_option in &mut self.0 {
            if frame_option.is_some() {
                return frame_option.take();
            }
        }

        None
    }
}

impl FrameDeallocator<Size4KiB> for TinyAllocator {
    unsafe fn deallocate_frame(&mut self, frame: PhysFrame) {
        for frame_option in &mut self.0 {
            if frame_option.is_none() {
                *frame_option = Some(frame);
                return;
            }
        }

        panic!("Tiny allocator can hold only 3 frames.");
    }
}

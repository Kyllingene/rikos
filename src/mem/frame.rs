use multiboot2::{ElfSection, ElfSectionIter, MemoryArea, MemoryAreaType};
use x86_64::{
    // registers::control::Cr3,
    structures::paging::{FrameAllocator, PageTable, PhysFrame, RecursivePageTable, Size4KiB},
    PhysAddr,
    // VirtAddr,
};

// use crate::interrupt::apic::LAPIC_START;

pub const P4: *mut PageTable = 0xffffffff_fffff000 as *mut _;

pub struct BootInfoFrameAllocator {
    mem_map: &'static [MemoryArea],
    next: usize,
    elf_section: Option<ElfSection>,
    elf_sections: ElfSectionIter,
}

impl BootInfoFrameAllocator {
    pub unsafe fn init(mem_map: &'static [MemoryArea], mut elf_sections: ElfSectionIter) -> Self {
        let elf_section = elf_sections.next();
        BootInfoFrameAllocator {
            mem_map,
            next: 0,
            elf_section,
            elf_sections,
        }
    }

    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        let usable_regions = self.mem_map.iter().filter(|r| {
            MemoryAreaType::from(r.typ()) == MemoryAreaType::Available
            // TODO: is this needed?
            // && !((r.start_address()..r.end_address()).contains(&LAPIC_START))
        });

        let addr_ranges = usable_regions.map(|r| r.start_address()..r.end_address());
        let frame_addrs = addr_ranges.flat_map(|r| r.step_by(4096));

        frame_addrs.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let mut frames = self.usable_frames();
        let mut new_frame = frames.nth(self.next);

        if let Some(section) = &self.elf_section {
            if let Some(mut frame) = &new_frame {
                let mut end = frame.start_address() + frame.size();
                let mut range = frame.start_address()..end;
                while range.contains(&PhysAddr::new(section.start_address()))
                    || range.contains(&PhysAddr::new(section.end_address()))
                {
                    end = frame.start_address() + frame.size();
                    range = frame.start_address()..end;
                    frame = frames.next()?;
                    self.next += 1;
                }

                new_frame = Some(frame);
                self.elf_section = self.elf_sections.next();
            }
        }

        self.next += 1;
        new_frame
    }
}

// unsafe fn active_lv4_table(phys_mem_offset: VirtAddr) -> &'static mut PageTable {
//     let (lv4_tf, _) = Cr3::read();

//     let phys = lv4_tf.start_address();
//     let virt = phys_mem_offset + phys.as_u64();
//     let pt_ptr: *mut PageTable = virt.as_mut_ptr();

//     &mut *pt_ptr
// }

pub unsafe fn init() -> RecursivePageTable<'static> {
    RecursivePageTable::new(unsafe { &mut *(P4 as *mut _) })
        .expect("failed to make recursive page table")
}

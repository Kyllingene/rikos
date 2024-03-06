#![allow(unused)]
use crate::println;

use conquer_once::spin::OnceCell;
use pic8259::ChainedPics;
use spin::Mutex;
// use x2apic::ioapic::{IoApic, IrqFlags, IrqMode, RedirectionTableEntry};
// use x2apic::lapic::{xapic_base, LocalApic, LocalApicBuilder, TimerDivide};
use x86_64::{
    instructions::port::Port,
    structures::paging::{FrameAllocator, Mapper, Page, PageTableFlags, PhysFrame, Size4KiB},
    PhysAddr, VirtAddr,
};

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
    Error,
    Spurious = PIC_1_OFFSET + 7,
}

// TODO: are these needed?
// pub const LAPIC_START: u64 = 0x4444_0000_0000;
// pub const IOAPIC_START: u64 = 0x4444_1000_0000;

// const APIC_OFFSET: usize = 32;
//
//
// pub(crate) static LAPIC: OnceCell<Mutex<LocalApic>> = OnceCell::uninit();
// pub(crate) static IOAPIC: OnceCell<Mutex<IoApic>> = OnceCell::uninit();
//
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// #[repr(u8)]
// pub enum InterruptIndex {
//     Timer = APIC_OFFSET,
//     Keyboard,
//     Error,
//     Spurious = APIC_OFFSET + 7,
// }
//
// fn disable_pic() {
//     /*
//         mov al, 0xff
//         out 0xa1, al
//         out 0x21, al
//     */
//
//     let mut port1 = Port::new(0xa1);
//     unsafe { port1.write(0xffu8) };
//     let mut port2 = Port::new(0x21);
//     unsafe { port2.write(0xffu8) };
// }
//
// fn init_apic(
//     mapper: &mut impl Mapper<Size4KiB>,
//     frame_allocator: &mut impl FrameAllocator<Size4KiB>,
// ) {
//     disable_pic();
//
//     let apic_phys_addr: u64 = unsafe { xapic_base() };
//
//     let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
//     unsafe {
//         mapper
//             .map_to(
//                 Page::containing_address(VirtAddr::new(LAPIC_START)),
//                 PhysFrame::containing_address(PhysAddr::new(apic_phys_addr)),
//                 flags,
//                 frame_allocator,
//             )
//             .expect("failed to map lapic")
//             .flush();
//     }
//
//     let mut lapic = LocalApicBuilder::new()
//         .timer_vector(InterruptIndex::Timer as usize)
//         .error_vector(InterruptIndex::Error as usize)
//         .spurious_vector(InterruptIndex::Spurious as usize)
//         .set_xapic_base(LAPIC_START)
//         .build()
//         .expect("failed to build lapic");
//
//     unsafe {
//         lapic.set_timer_divide(TimerDivide::Div16);
//         lapic.enable();
//     }
//
//     LAPIC.init_once(|| Mutex::new(lapic));
//
//     let frame = frame_allocator
//         .allocate_frame()
//         .expect("failed to allocate frame for ioapic");
//
//     unsafe {
//         mapper
//             .map_to(
//                 Page::containing_address(VirtAddr::new(IOAPIC_START)),
//                 frame,
//                 flags,
//                 frame_allocator,
//             )
//             .expect("failed to map ioapic")
//             .flush();
//     }
//
//     unsafe {
//         let mut ioapic = IoApic::new(IOAPIC_START);
//
//         ioapic.init(APIC_OFFSET as u8);
//
//         let mut entry = RedirectionTableEntry::default();
//         entry.set_mode(IrqMode::Fixed);
//         entry.set_flags(IrqFlags::LEVEL_TRIGGERED | IrqFlags::LOW_ACTIVE | IrqFlags::MASKED);
//         entry.set_dest(0);
//
//         ioapic.set_table_entry(0, entry);
//
//         ioapic.enable_irq(0);
//
//         IOAPIC.init_once(|| Mutex::new(ioapic));
//     }
// }

pub fn init(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    println!("initializing apic");
    // init_apic(mapper, frame_allocator);
    unsafe { PICS.lock().initialize() };

    println!("enabling interrupts");
    x86_64::instructions::interrupts::enable();
}

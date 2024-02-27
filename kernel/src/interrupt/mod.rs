use x86_64::structures::{
    idt::InterruptDescriptorTable,
    paging::{FrameAllocator, Mapper, Size4KiB},
};

use lazy_static::lazy_static;

pub(crate) mod apic;
mod exception;
pub mod input;

use apic::InterruptIndex;
use input::handlers::*;

use crate::{mem::gdt, println};

use exception::*;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        // exceptions
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        // interrupts
        idt[InterruptIndex::Timer as usize].set_handler_fn(timer_handler);
        idt[InterruptIndex::Keyboard as usize].set_handler_fn(keyboard_handler);
        idt[InterruptIndex::Error as usize].set_handler_fn(error_handler);
        idt[InterruptIndex::Spurious as usize].set_handler_fn(spurious_handler);

        idt
    };
}

pub fn init(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    println!("initializing idt");
    IDT.load();
    println!("initializing input");
    apic::init(mapper, frame_allocator);
}

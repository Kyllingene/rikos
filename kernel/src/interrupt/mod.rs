use conquer_once::spin::OnceCell;
use x86_64::structures::{
    idt::InterruptDescriptorTable,
    paging::{FrameAllocator, Mapper, Size4KiB},
};

pub(crate) mod apic;
mod exception;
pub mod input;

use apic::InterruptIndex;
use input::handlers;

use crate::{mem::gdt, println};

static IDT: OnceCell<InterruptDescriptorTable> = OnceCell::uninit();

fn init_idt() -> InterruptDescriptorTable {
    let mut idt = InterruptDescriptorTable::new();

    // exceptions
    idt.breakpoint.set_handler_fn(exception::breakpoint_handler);
    idt.page_fault.set_handler_fn(exception::page_fault_handler);
    unsafe {
        idt.double_fault
            .set_handler_fn(exception::double_fault_handler)
            .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
    }

    // interrupts
    idt[InterruptIndex::Timer as usize].set_handler_fn(handlers::timer);
    idt[InterruptIndex::Keyboard as usize].set_handler_fn(handlers::keyboard);
    idt[InterruptIndex::Error as usize].set_handler_fn(handlers::error);
    idt[InterruptIndex::Spurious as usize].set_handler_fn(handlers::spurious);

    idt
}

pub fn init(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    println!("initializing idt");
    IDT.get_or_init(init_idt).load();

    println!("initializing input");
    apic::init(mapper, frame_allocator);
}

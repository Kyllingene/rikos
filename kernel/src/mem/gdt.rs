//use core::cell::OnceCell;
use conquer_once::spin::OnceCell;

use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

use crate::println;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 6;

static TSS: OnceCell<TaskStateSegment> = OnceCell::uninit();

fn init_tss() -> TaskStateSegment {
    let mut tss = TaskStateSegment::new();
    tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
        const STACK_SIZE: usize = 4096 * 5;
        static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

        let stack_start = VirtAddr::from_ptr(unsafe { core::ptr::addr_of!(STACK) });
        let stack_end = stack_start + STACK_SIZE;
        stack_end
    };
    tss
}

static GDT: OnceCell<(GlobalDescriptorTable, Selectors)> = OnceCell::uninit();

fn init_gdt() -> (GlobalDescriptorTable, Selectors) {
    let mut gdt = GlobalDescriptorTable::new();

    let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());

    println!("initializing tss");
    let tss_selector = gdt.add_entry(Descriptor::tss_segment(TSS.get_or_init(init_tss)));

    (
        gdt,
        Selectors {
            code_selector,
            tss_selector,
        },
    )
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub fn init() {
    use x86_64::instructions::segmentation::{Segment, CS, SS};
    use x86_64::instructions::tables::load_tss;

    let gdt = GDT.get_or_init(init_gdt);
    gdt.0.load();
    unsafe {
        SS::set_reg(SegmentSelector(0));
        CS::set_reg(gdt.1.code_selector);
        load_tss(gdt.1.tss_selector);
    }
}

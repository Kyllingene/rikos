use x86_64::{
    registers::control::Cr2,
    structures::idt::{InterruptStackFrame, PageFaultErrorCode},
};

use crate::serial_println;

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    serial_println!("BREAKPOINT\nstack frame: {stack_frame:#?}");
}

pub extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    serial_println!("page fault");
    serial_println!("address: {:?}", Cr2::read());
    serial_println!("error code: {:?}", error_code);
    serial_println!("stack frame: {:#?}", stack_frame);

    loop {
        x86_64::instructions::hlt();
    }
}

pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _code: u64,
) -> ! {
    panic!("unhandled exception\n{stack_frame:#?}");
}

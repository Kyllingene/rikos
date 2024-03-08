use circular_buffer::CircularBuffer;
use core::sync::atomic::AtomicU64;
use spin::Mutex;

pub static TIMER_COUNT: AtomicU64 = AtomicU64::new(0);
pub static SCANCODE_QUEUE: Mutex<CircularBuffer<128, u8>> = Mutex::new(CircularBuffer::new());

use crate::interrupt::apic::{InterruptIndex, PICS};
use crate::serial_println;

use x86_64::instructions::port::Port;
use x86_64::structures::idt::InterruptStackFrame;

fn add_scancode(code: u8) {
    let mut queue = SCANCODE_QUEUE.lock();
    if queue.try_push_back(code).is_err() {
        #[cfg(debug_assertions)]
        serial_println!("WARNING: scancode queue full, dropping keyboard input");
    }
}

fn eoi(irq: InterruptIndex) {
    unsafe {
        PICS.lock().notify_end_of_interrupt(irq as u8);
    }
    // unsafe {
    //     crate::interrupt::apic::LAPIC
    //         .get()
    //         .expect("lapic uninitialized")
    //         .lock()
    //         .end_of_interrupt();
    // }
}

pub extern "x86-interrupt" fn timer(_stack_frame: InterruptStackFrame) {
    TIMER_COUNT.fetch_add(1, core::sync::atomic::Ordering::SeqCst);
    eoi(InterruptIndex::Timer);
}

pub extern "x86-interrupt" fn keyboard(_stack_frame: InterruptStackFrame) {
    let mut port = Port::new(0x60);
    let code: u8 = unsafe { port.read() };
    add_scancode(code);

    let mut port = Port::new(0x20);
    unsafe { port.write(0x20_u8) };

    eoi(InterruptIndex::Keyboard);
}

pub extern "x86-interrupt" fn error(_stack_frame: InterruptStackFrame) {
    panic!("APIC error");
}

pub extern "x86-interrupt" fn spurious(_stack_frame: InterruptStackFrame) {
    serial_println!("spurious interrupt");
    // eoi();
}

use core::sync::atomic::{AtomicU64, Ordering};

use crate::serial_println;

static TIMER_COUNT: AtomicU64 = AtomicU64::new(0);

/// Sleeps for a certain number of milliseconds.
#[allow(unused)]
pub fn sleep(millis: u64) {
    TIMER_COUNT.store(0, Ordering::SeqCst);

    while TIMER_COUNT.load(Ordering::SeqCst) < millis {
        serial_println!("waiting {}", TIMER_COUNT.load(Ordering::SeqCst));
        x86_64::instructions::hlt();
    }
}

pub(super) mod handlers {
    use crate::println;

    use crate::interrupt::apic::InterruptIndex;
    use crate::interrupt::apic::PICS;
    use crate::interrupt::input::TIMER_COUNT;
    use crate::serial_println;

    use x86_64::instructions::port::Port;
    use x86_64::structures::idt::InterruptStackFrame;

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
        let tick = TIMER_COUNT.fetch_add(1, core::sync::atomic::Ordering::SeqCst);
        println!("{tick}");
        eoi(InterruptIndex::Timer);
    }

    pub extern "x86-interrupt" fn keyboard(_stack_frame: InterruptStackFrame) {
        let mut port = Port::new(0x60);
        let scancode: u8 = unsafe { port.read() };
        crate::task::keyboard::add_scancode(scancode);

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
}

use core::task::Poll;

use futures_util::Future;
use x86_64::instructions::port::Port;

static mut TIMER_COUNT: usize = 0;

/// Sleeps for a certain number of milliseconds.
///
/// Returns a [`Sleeper`], which implements [`Future`].
#[allow(unused)]
pub fn sleep(millis: usize) -> impl Future<Output = ()> {
    unsafe { TIMER_COUNT = 0 };

    let mut channel_port = Port::new(0x43);
    unsafe { channel_port.write(0x00 as u8) };

    let mut divisor_port = Port::new(0x40);
    unsafe { divisor_port.write(0xa9 as u8) };
    unsafe { divisor_port.write(0x04 as u8) };

    Sleeper(millis)
}

/// impl [`Future`] that returns [`Poll::Ready`] when a certain
/// number of milliseconds have passed. Create via [`sleep`].
pub struct Sleeper(usize);

impl Future for Sleeper {
    type Output = ();

    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        if unsafe { TIMER_COUNT } >= self.0 {
            cx.waker().wake_by_ref();
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

pub(super) mod handlers {
    use crate::serial_println;

    use x86_64::instructions::port::Port;
    use x86_64::structures::idt::InterruptStackFrame;

    fn eoi() {
        unsafe {
            crate::interrupt::apic::LAPIC
                .get()
                .expect("lapic uninitialized")
                .lock()
                .end_of_interrupt();
        }
    }

    pub extern "x86-interrupt" fn timer_handler(_stack_frame: InterruptStackFrame) {
        unsafe { super::TIMER_COUNT = super::TIMER_COUNT.wrapping_add(1) };
        eoi();
    }

    pub extern "x86-interrupt" fn keyboard_handler(_stack_frame: InterruptStackFrame) {
        let mut port = Port::new(0x60);
        let scancode: u8 = unsafe { port.read() };
        crate::task::keyboard::add_scancode(scancode);

        eoi();
    }

    pub extern "x86-interrupt" fn error_handler(_stack_frame: InterruptStackFrame) {
        panic!("APIC error");
    }

    pub extern "x86-interrupt" fn spurious_handler(_stack_frame: InterruptStackFrame) {
        serial_println!("spurious interrupt");
        // eoi();
    }
}

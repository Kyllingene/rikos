use core::sync::atomic::Ordering;

use crate::interrupt::handlers::TIMER_COUNT;
use crate::serial_println;

/// Waits for user input, or a timer tick.
///
/// Use in loops waiting for input.
pub fn wait() {
    x86_64::instructions::hlt();
}

/// Sleeps for a certain number of milliseconds.
#[allow(unused)]
pub fn sleep(millis: u64) {
    TIMER_COUNT.store(0, Ordering::SeqCst);

    while TIMER_COUNT.load(Ordering::SeqCst) < millis {
        serial_println!("sleep: {:p}", core::ptr::addr_of!(TIMER_COUNT));
        x86_64::instructions::hlt();
    }
}

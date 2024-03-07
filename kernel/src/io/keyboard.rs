use crate::serial_println;

use crate::interrupt::handlers::SCANCODE_QUEUE;

pub fn getch() -> Option<u8> {
    SCANCODE_QUEUE.lock().pop_front()
}

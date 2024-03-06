use circular_buffer::CircularBuffer;
use spin::mutex::Mutex;

use crate::serial_println;

static SCANCODE_QUEUE: Mutex<CircularBuffer<128, u8>> = Mutex::new(CircularBuffer::new());

pub fn add_scancode(scancode: u8) {
    let mut queue = SCANCODE_QUEUE.lock();
    if let Err(_) = queue.try_push_back(scancode) {
        #[cfg(debug_assertions)]
        serial_println!("WARNING: scancode queue full, dropping keyboard input");
    }
}

pub fn getch() -> Option<u8> {
    serial_println!("len: {}", SCANCODE_QUEUE.lock().len());
    SCANCODE_QUEUE.lock().pop_front()
}

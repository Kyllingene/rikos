#![no_std]

extern crate alloc;

use core::future::{Future, self};

use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use futures_util::StreamExt;
use kernel::interrupt::input::sleep;
use spin::Mutex;

use kernel::task::{executor::Executor, keyboard::ScancodeStream, Task};

use kernel::{print, serial_println, println};

mod os;
mod test;

static mut KEYBOARD_HANDLER: Mutex<Option<fn(u8)>> = Mutex::new(None);

static mut RUNTIME_TASKS: OnceCell<ArrayQueue<Task>> = OnceCell::uninit();

/// Register a new keyboard handler, replacing any previous one.
fn set_keyboard_handler(f: fn(u8)) {
    *unsafe { KEYBOARD_HANDLER.lock() } = Some(f);
}

/// Spawn a new task in the OS task pool.
/// 
/// If the intermediate pool is full, returns false.
fn spawn_new_task(task: impl Future<Output = ()> + 'static) -> bool {
    match unsafe { RUNTIME_TASKS.get() }.expect("runtime tasks uninitialized").push(Task::new(task)) {
        Ok(()) => true,
        Err(_) => false,
    }
}

async fn handle_keyboard() {
    let mut scancode_stream = ScancodeStream::new();

    while let Some(ch) = scancode_stream.next().await {
        if let Some(f) = unsafe { KEYBOARD_HANDLER.lock() }.as_ref() {
            f(ch);
        }
    }
}

async fn sleep_dot() {
    loop {
        // sleep(1000).await;
        future::pending::<()>().await;
        print!(".");
    }
}

#[no_mangle]
extern "C" fn os_main() {
    unsafe { RUNTIME_TASKS.init_once(|| ArrayQueue::new(128)) };

    #[cfg(test)]
    {
        serial_println!("\nrunning os tests");
        test::run();
        return;
    }

    let mut executor = Executor::new();
    executor.spawn(Task::new(os::main()));
    // executor.spawn(Task::new(sleep_dot()));
    executor.spawn(Task::new(handle_keyboard()));
    executor.run();
}

extern crate alloc;

use kernel::{
    print, println,
    vga::{self, Color},
};

use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, KeyCode, Keyboard, ScancodeSet1};
use spin::Mutex;

use crate::set_keyboard_handler;

fn handle_keypress(scancode: u8) {
    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(
                ScancodeSet1::new(),
                layouts::Us104Key,
                HandleControl::Ignore,
            ));
    }

    let mut keyboard = KEYBOARD.lock();
    if let Ok(Some(ev)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(ev) {
            match key {
                DecodedKey::Unicode(ch) => match ch {
                    '\x08' => vga::backspace(),
                    _ => print!("{ch}"),
                },
                DecodedKey::RawKey(key) => match key {
                    KeyCode::Backspace => vga::backspace(),
                    KeyCode::Home => vga::home(),
                    KeyCode::End => vga::end(),
                    KeyCode::Return | pc_keyboard::KeyCode::NumpadEnter => vga::enter(),
                    KeyCode::ArrowLeft => vga::left(),
                    KeyCode::ArrowRight => vga::right(),
                    _ => {}
                },
            }
        }
    }
}

fn welcome() {
    vga::clear();
    #[cfg(debug_assertions)]
    {
        vga::set_fg(Color::Brown);
        println!("use ctrl-alt-g to release mouse from qemu\n");
    }

    vga::set_fg(Color::LightBlue);
    print!("\t welcome to ");
    vga::set_fg(Color::LightRed);
    println!("rikOS");
    vga::reset();
}

pub async fn main() {
    set_keyboard_handler(handle_keypress);

    welcome();
}

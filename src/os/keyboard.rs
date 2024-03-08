#![allow(unused)]

use conquer_once::spin::OnceCell;
use pc_keyboard::{layouts, DecodedKey, HandleControl, KeyCode, Keyboard, ScancodeSet1};
use spin::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    Enter,
    Backspace,
    Char(char),
}

pub fn getch() -> Option<Key> {
    static KEYBOARD: OnceCell<Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>>> =
        OnceCell::uninit();

    let init_kbd = || {
        Mutex::new(Keyboard::new(
            ScancodeSet1::new(),
            layouts::Us104Key,
            HandleControl::Ignore,
        ))
    };

    let mut keyboard = KEYBOARD.get_or_init(init_kbd).lock();
    let ev = keyboard.add_byte(crate::io::keyboard::getch()?).ok()??;
    let key = keyboard.process_keyevent(ev)?;
    Some(match key {
        DecodedKey::Unicode(ch) => match ch {
            '\x08' => Key::Backspace,
            _ => Key::Char(ch),
        },
        DecodedKey::RawKey(key) => match key {
            KeyCode::Backspace => Key::Backspace,
            KeyCode::Home => Key::Home,
            KeyCode::End => Key::End,
            KeyCode::Return | KeyCode::NumpadEnter => Key::Enter,
            KeyCode::ArrowUp => Key::Up,
            KeyCode::ArrowDown => Key::Down,
            KeyCode::ArrowLeft => Key::Left,
            KeyCode::ArrowRight => Key::Right,
            _ => return None,
        },
    })
}

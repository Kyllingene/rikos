#![no_std]

extern crate alloc;

mod keyboard;

use kernel::{
    print, println, serial_println,
    vga::{self, Color},
};
use keyboard::getch;

use crate::keyboard::Key;

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

#[no_mangle]
extern "C" fn os_main() {
    welcome();

    // loop {
    if let Some(key) = getch() {
        serial_println!("{key:?}");

        if let Key::Char(ch) = key {
            print!("{ch}");
        }
    }
    // }
}

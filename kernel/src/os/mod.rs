mod keyboard;

#[allow(unused_imports)]
use crate::{
    io::timer::{sleep, wait},
    print, println,
    vga::{self, Color},
};
use keyboard::getch;

use keyboard::Key;

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

pub fn main() {
    welcome();

    loop {
        wait();
        if let Some(key) = getch() {
            if let Key::Char(ch) = key {
                print!("{ch}");
            }
        }
    }
}

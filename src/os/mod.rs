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

fn screensave() {
    let prev = vga::save();

    vga::clear();
    vga::set_fg(Color::Yellow);
    println!("press any key to wake up");

    loop {
        wait();
        if getch().is_some() {
            break;
        }
    }

    vga::restore(prev);
}

pub fn main() {
    welcome();

    loop {
        wait();
        if let Some(key) = getch() {
            match key {
                Key::Char(ch) => print!("{ch}"),
                Key::Up => todo!(),
                Key::Down => todo!(),
                Key::Left => todo!(),
                Key::Right => todo!(),
                Key::Home => todo!(),
                Key::End => screensave(),
                Key::Enter => println!(),
                Key::Backspace => {
                    vga::left();
                    print!(" ");
                    vga::left();
                }
            }
        }
    }
}

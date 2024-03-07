#![allow(internal_features)]
#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(lang_items)]

// TODO: find out how to remove this
extern crate alloc;

pub mod interrupt;

#[macro_use]
pub mod io;

pub mod mem;
mod panic;

#[cfg(test)]
mod test;

use conquer_once::spin::OnceCell;

pub use io::vga;
use mem::frame::BootInfoFrameAllocator;
use multiboot2::{BootInformation, BootInformationHeader};

static BOOT_INFO: OnceCell<BootInformation<'static>> = OnceCell::uninit();

#[no_mangle]
extern "C" fn kernel_main(multiboot_info_addr: usize) {
    println!("initializing kernel");
    println!("loading multiboot2 info");

    let boot_info = BOOT_INFO.get_or_init(|| {
        unsafe { BootInformation::load(multiboot_info_addr as *const BootInformationHeader) }
            .expect("failed to load boot info")
    });

    println!("loading memory map");
    let memory_map = boot_info
        .memory_map_tag()
        .expect("failed to get memory map");
    let memory_areas = memory_map.memory_areas();

    let elf_sections = boot_info
        .elf_sections()
        .expect("failed to get elf sections");

    println!("initializing frame allocator");
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(memory_areas, elf_sections) };

    println!("initializing mapper");
    let mut mapper = unsafe { mem::frame::init() };

    println!("initializing interrupts");
    // interrupt::init(&mut mapper, &mut frame_allocator);
    interrupt::init();

    println!("initializing memory");
    mem::init();

    println!("initializing heap");
    mem::allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("failed to initialize heap");

    #[cfg(test)]
    {
        println!("running kernel tests");
        test::run();
    }

    serial_println!(
        "kernel addr of TIMER_COUNT: {:p}",
        core::ptr::addr_of!(crate::interrupt::handlers::TIMER_COUNT)
    );

    println!("passing to OS");
    // TODO: should this be changed?
}

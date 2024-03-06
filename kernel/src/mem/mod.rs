use crate::println;

pub(crate) mod allocator;
pub(crate) mod frame;
pub(crate) mod gdt;

pub use allocator::HEAP_SIZE;

// TODO: is this necessary?
// https://os.phil-opp.com/remap-the-kernel/
// pub(crate) mod remap;

pub fn init() {
    println!("loading gdt");
    gdt::init();
}

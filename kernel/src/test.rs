#![cfg(test)]

use alloc::boxed::Box;

use crate::{mem::allocator::HEAP_SIZE, serial_print, serial_println};

static TESTS: [fn(); 3] = [basic_assertion, many_boxes, many_boxes_with_long_lived];

pub trait Testable: Sync {
    fn run_test(&self);
}

impl Testable for fn() {
    fn run_test(&self) {
        serial_print!("test {}...\t", core::any::type_name::<Self>());
        self();
        serial_println!("ok");
    }
}

pub fn run() {
    serial_println!("running {} tests", TESTS.len());
    for test in TESTS {
        test.run_test();
    }
}

fn basic_assertion() {
    assert_eq!(1, 1);
    assert!(1 != 2);
}

fn many_boxes_with_long_lived() {
    let long_lived_box = Box::new(125);
    for i in 0..HEAP_SIZE {
        let short_box = Box::new(i);
        assert_eq!(*long_lived_box, 125);
        assert_eq!(*short_box, i);
    }

    assert_eq!(*long_lived_box, 125);
}

fn many_boxes() {
    for i in 0..HEAP_SIZE {
        let short_box = Box::new(i);
        assert_eq!(*short_box, i);
    }
}

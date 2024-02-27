#![cfg(test)]

extern crate alloc;

use alloc::boxed::Box;

use kernel::{serial_print, serial_println};

static TESTS: [fn(); 3] = [basic_assertion, many_boxes, many_boxes_with_long_lived];

pub fn run() {
    serial_println!("running {} tests", TESTS.len());
    for test in TESTS {
        test();
        serial_println!("ok");
    }
}

fn basic_assertion() {
    serial_print!("test basic_assertion...\t");
    assert_eq!(1, 1);
    assert!(1 != 2);
}

fn many_boxes_with_long_lived() {
    serial_print!("test many_boxes_with_long_lived...\t");
    let long_lived_box = Box::new(125);
    for i in 0..100 * 1024 {
        let short_box = Box::new(i);
        assert_eq!(*long_lived_box, 125);
        assert_eq!(*short_box, i);
    }

    assert_eq!(*long_lived_box, 125);
}

fn many_boxes() {
    serial_print!("test many_boxes...\t");
    for i in 0..100 * 1024 {
        let short_box = Box::new(i);
        assert_eq!(*short_box, i);
    }
}

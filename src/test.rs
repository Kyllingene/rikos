#![allow(unused)]

use alloc::boxed::Box;

use crate::{mem::allocator::HEAP_SIZE, serial_print, serial_println};

macro_rules! tests {
    ( $( $test:ident ),* $(,)? ) => {
        static TESTS: &[(&'static str, fn())] = &[$(
            (stringify!($test), $test)
        ),*];
    };
}

tests! {
    many_boxes_with_long_lived,
    many_boxes,
}

pub trait Testable: Sync {
    fn run_test(&self);
}

impl Testable for (&'static str, fn()) {
    fn run_test(&self) {
        serial_print!("test {}...\t", self.0);
        (self.1)();
        serial_println!("ok");
    }
}

pub fn run() {
    serial_println!("running {} tests", TESTS.len());
    for test in TESTS {
        test.run_test();
    }
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

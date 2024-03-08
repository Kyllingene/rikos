//#[cfg(not(test))]
use core::panic::PanicInfo;

//#[cfg(not(debug_assertions))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::println;

    println!("error: {}", info);

    loop {
        x86_64::instructions::hlt();
    }
}

// TODO: fix complaint about std from test
// #[cfg(not(test))]
// #[cfg(debug_assertions)]
// #[panic_handler]
// fn panic(info: &PanicInfo) -> ! {
//     use crate::serial_println;
//
//     serial_println!("error: {}", info);
//
//     loop {
//         x86_64::instructions::hlt();
//     }
// }

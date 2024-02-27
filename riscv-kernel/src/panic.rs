use core::panic::PanicInfo;

use crate::println;

#[lang="eh_personality"]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {

    println!("error: {}", info);

    loop {
        unsafe {
            riscv::asm::wfi();
        }
    }
}


#![no_std] // Do not link std lib
#![no_main] // Disable Rust entry points

use core::panic::PanicInfo;

static HELLO: &[u8] = &[0b1001000];  // H

#[no_mangle] // Do not mangle name
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default

    let vga_buffer = 0xb8200 as *mut u8; // Positioning

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

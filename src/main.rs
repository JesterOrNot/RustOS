#![no_std] // Do not link std lib
#![no_main] // Disable Rust entry points

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World From RustOS!";

#[no_mangle] // Do not mangle name
pub extern "C" fn _start() -> ! {
    // this method is the entry point, since the linker looks for a method
    // named `_start` by default

    let vga_text_buffer = 0xb8000 as *mut u8; // This is the memory adress of the vga_text_buffer

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_text_buffer.offset(i as isize * 2) = byte;
            *vga_text_buffer.offset(i as isize * 2 + 1) = 0xa;
        }
    }

    loop {}
}

/// This method is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

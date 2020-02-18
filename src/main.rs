#![no_std] // Do not link std lib
#![no_main] // Disable Rust entry points
use core::panic::PanicInfo;

#[no_mangle] // Do not mangle name
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    loop {}
}

// Call this function on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

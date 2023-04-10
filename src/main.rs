// Disable the standard library since the OS cannot depend on our current OS
// Now we need a panic handler and a language item

#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

/// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// The OS entry point
#[no_mangle] // So the name "_start()" of the function is not mangled
pub extern "C" fn _start() -> ! {
    // Cast an integer to a raw pointer
    let vga_buffer = 0xb8000 as *mut u8;

    // Iterate over the bytes of static byte string
    for (i, &byte) in HELLO.iter().enumerate() {
        // Raw pointers are always unsafe! Avoid this if possible
        unsafe {
            // Write to the VGA Buffer
            *vga_buffer.offset(i as isize * 2) = byte;
            // The color bit, 0xd is light purple
            *vga_buffer.offset(i as isize * 2 + 1) = 0xd;
        }
    }

    loop {}
}

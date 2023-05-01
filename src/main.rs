// Disable the standard library since the OS cannot depend on our current OS
// Now we need a panic handler and a language item

#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer; // for printing

static HELLO: &[u8] = b"Hello World!";

/// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// The OS entry point
#[no_mangle] // So the name "_start()" of the function is not mangled
pub extern "C" fn _start() -> ! {
    // Test the print function
    //vga_buffer::print_something();
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello world again!").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 123, 5.7).unwrap();

    loop {}
}

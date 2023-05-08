// Disable the standard library since the OS cannot depend on our current OS
// Now we need a panic handler and a language item

#![no_std]
#![no_main]

// Custom test framework
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

use core::panic::PanicInfo;

mod vga_buffer; // for printing

static HELLO: &[u8] = b"Hello World!";

/// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// The first test case to test if 1 equals to 1
#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

// Define some exit codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

// Specify the exit status
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

// Test framework
#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    // Exit after the tests are finished
    exit_qemu(QemuExitCode::Success);
}

/// The OS entry point
#[no_mangle] // So the name "_start()" of the function is not mangled
pub extern "C" fn _start() -> ! {
    // Updated print method
    println!("Hello World{}", "!");

    // Run the custom test framework
    #[cfg(test)]
    test_main();

    loop {}
}

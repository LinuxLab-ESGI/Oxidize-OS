
#![no_std] // Disabling the standard library
#![no_main] // Disabling the Rust-level entry point

// Handling panic messages
use core::panic::PanicInfo;

// Diverging function called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static STRING: &[u8] = b"Oxidize OS";

// Entrypoint (Start of the program)
#[no_mangle] // Disabling name mangling, allowing the name to be used by the linker
pub extern "C" fn _start() -> ! { // pub extern "C" to allow calling from C
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in STRING.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xf;
        }
    }
    loop{}
}

// To build the project and run it:
// cargo build
// cargo run

// Launching the project:
// qemu-system-x86_64 -drive format=raw,file=target/x86_64-oxidize_os/debug/bootimage-oxidize_os.bin

// Writing in an usb device:
// dd if=target/x86_64-oxidize_os/debug/bootimage-oxidize_os.bin of=/dev/sdX && sync
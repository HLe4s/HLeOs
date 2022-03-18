#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;
mod hleos;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    static HELLO: &[u8] = b"Hello World!";
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    for _i in 0..100000000 {}

    let vga : &hleos::vga::VgaHandle = hleos::vga::get_vga_handle();

    vga.clear();
    
    loop{}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

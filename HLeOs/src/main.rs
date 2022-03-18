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

	print_test();
    
    loop{}
}

fn print_test() {
   let vga : &hleos::vga::VgaHandle = hleos::vga::get_vga_handle();

    vga.clear();

    vga.print_char(b'x');
    vga.print_char(b'y');
    vga.print_char(b'z');

    for _i in 0..10000000 {}

    vga.delete_char();
    for _i in 0..10000000 {}
    vga.delete_char();
    for _i in 0..10000000 {}
    vga.delete_char();
    for _i in 0..10000000 {}

    vga.print_char(b'H');
    for _i in 0..10000000 {}
    vga.print_char(b'e');
    for _i in 0..10000000 {}
    vga.print_char(b'l');
    for _i in 0..10000000 {}
    vga.print_char(b'l');
    for _i in 0..10000000 {}
    vga.print_char(b'o');
    for _i in 0..10000000 {}
    vga.print_char(b'\n');
    for _i in 0..10000000 {}
    vga.print_char(b'w');
    for _i in 0..10000000 {}
    vga.print_char(b'o');
    for _i in 0..10000000 {}
    vga.print_char(b'r');
    for _i in 0..10000000 {}
    vga.print_char(b'l');
    for _i in 0..10000000 {}
    vga.print_char(b'd');
    for _i in 0..10000000 {}
    vga.print_char(b'!');
    for _i in 0..10000000 {}
    vga.print_char(b'\n');
    for _i in 0..10000000 {}

    vga.print_line(b"Hello, This is my HLeOs. \n");
    for _i in 0..100000000 {}
    vga.delete_char();
    vga.delete_line();
    for _i in 0..10000000 {}
    vga.print_line(b"Hello, This is my HLeOs. \n");
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

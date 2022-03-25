#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;
mod hleos;
mod std;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print_test();

    println!("\nHello my name is Kang ChanU\n", "my student number is : ", 202020696);
    println!("yes you are my friend too!");

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

    vga.print_line(b"Q : What is... ");
    vga.print_number(854952);
    vga.print_line(b" plus ");
    vga.print_number(32432);
    vga.print_char(b'?');
    for _i in 0..10000000 {}
    vga.print_char(b'\n');
    vga.print_line(b"A : ");
    vga.print_number(854952 + 32432);
    vga.print_line(b", sir. \n");
    for _i in 0..100000000 {}

    vga.print_line(b"Hello, I love study about computer science espicially, Operating system!!\n");
    for _i in 0..100000000 {}
    vga.delete_char();
    vga.delete_line();
    for _i in 0..10000000 {}
    vga.print_line(b"Hello, This is my HLeOs. \n");
    vga.print_line(b"I'm working on 3rd Chapter about printing something! \n");
    vga.print_line(b"Visit my github, and blog for more information! \n");
    vga.print_line(b"github : https://github.com/HLe4s/HLeOs \n");
    vga.print_line(b"blog : https://www.hacking-yi.kro.kr \n");

    println!(10, " + ", 324, " = ", 10 + 324);
    vga.delete_char();
    vga.delete_line();
    println!();
    println!(10, " + ", 324, " = ", 10 + 324);
    vga.print_line(b"HAHAHAHAH, Hello, I love study about computer science espicially, Operating system!!\n");
    vga.delete_char();
    for _i in 0..10000000 {}
    vga.delete_char();
    for _i in 0..10000000 {}
    vga.delete_char();
    for _i in 0..10000000 {}
    vga.delete_char();
    for _i in 0..10000000 {}
    vga.delete_char();
    for _i in 0..10000000 {}
    vga.delete_char();
    for _i in 0..10000000 {}
    vga.delete_char();
    for _i in 0..10000000 {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let vga : &hleos::vga::VgaHandle = hleos::vga::get_vga_handle();
    vga.print_line(b"Panic_eccured!!\n");
    loop {}
}

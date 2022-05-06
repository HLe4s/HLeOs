use super::hleos;
use super::std;
use super::println;
use super::print;

pub fn wait_a_little_bit() {
    for _i in 0..10000000 {}
}

pub fn wait_a_moment() {
    for _i in 0..10 {
        wait_a_little_bit();
    }
}


pub fn print_test() {
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
}

pub fn queue_test() {
    println!("\n[Let's play with toy queue!]\n");
    let mut q : std::queue::Queue = std::queue::Queue::new(0x16b00 as *mut u8, 1, 5);
    
    for i in 0..5 {
        let mut ch : u64 = 0x0;
        print!(i + 1 as i32, " : ");
        ch = hleos::iostream::getch() as u64;
        println!(ch as u8);
        q.enqueue(ch);
    }

    println!("I will pop it now..");
    for i in 0..5 {
        let mut ch : u64 = 0x0;
        ch = q.dequeue();
        println!(i + 1 as i32, " => ", ch as u8);
    }

    println!("end!");
}

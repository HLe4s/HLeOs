use super::super::iostream;
use super::super::asm::k_enable_interrupt;
use super::super::super::println;
use super::super::super::std;
use super::super::super::print;
use super::super::super::etc::wait_a_little_bit;
use super::super::super::etc::wait_a_moment;
use super::super::super::etc::hello;
use super::super::super::etc::th;

pub fn init() {
    println!("init job activated");
    k_enable_interrupt();
    loop {
        wait_a_moment();
        wait_a_moment();
        wait_a_moment();
        std::io::clear();
    }
}

pub fn print_hello() {
    k_enable_interrupt();
    loop {
        println!("Hello!");
        wait_a_little_bit();
    }
}

pub fn print_hi() {
    k_enable_interrupt();
    loop {
        println!("Hi!");
        wait_a_little_bit();
    }
}

pub fn getch_main() {
    k_enable_interrupt();
	loop {
        let mut ch : u8 = iostream::getch();
        if ch == b'*' {
            std::io::delete_a_char();
        } else {
            print!(ch);
        }
    }
}

pub fn dummy_main() {
    loop {

    }
}

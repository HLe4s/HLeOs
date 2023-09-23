use super::hleos;
use super::std;
use super::println;
use super::print;
use super::hleos::asm;
use super::std::bit_map::BitMap;
use super::hleos::thread;
use super::hleos::kmalloc;
use super::hleos::thread::jobs;

pub static mut th : u64 = 0x0;
pub static mut hello : bool = false;

extern "C" {
    pub fn memcpy(dst : *mut u8, src : *mut u8, n : i32);
}

pub fn wait_a_little_bit() {
    for _i in 0..10000000 {}
}

pub fn wait_a_moment() {
    for _i in 0..10 {
        wait_a_little_bit();
    }
}

pub fn thread_test() {
    let t = thread::create_thread(jobs::dummy_main, kmalloc::stack_kmalloc(0xff0));

    unsafe {
        th = thread::save_thread(t) as u64;
        if !hello {
            thread::load_thread(thread::create_thread(jobs::dummy_main, kmalloc::stack_kmalloc(0xff0)));
        }
    }

    for _i in 0..10 {
        println!("Hello!");
        wait_a_moment();
    }
} 

pub fn bit_map_test() {
    let len = 32;
    let mut bits : u64 = 0x0;
    let bit_map : BitMap = BitMap::new(&mut bits, len);

    bit_map.set_bit(15);
    bit_map.set_bit(30);

    println!(bit_map.find_first(true));
    bit_map.unset_bit(15);
    println!(bit_map.find_first(false));

    for i in 0..len {
        print!(i, " : ");
        print!(bit_map.is_set(i));

        if i % 3 == 2 {
            print!('\n');
        } else if i % 3 == 2 {
            print!("    ");
        } else {
            print!("    ");
        }
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


#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::arch::asm;
use core::panic::PanicInfo;
mod etc;
mod hleos;
mod std;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init_stack();
    init_interrupt();
    //etc::print_test();
    //etc::wait_a_moment();
    //std::io::clear();
   
    println!("hhihihiihihiihiihihih");
    cursor_print!(24, 10, "HELLOO");

    etc::queue_test();
    etc::wait_a_moment();
    std::io::clear();

    println!("There is nothing");

    loop{
        let vga : &hleos::vga::VgaHandle = hleos::vga::get_vga_handle();
        let ch : u8 = hleos::iostream::getch();
        if ch == b'*' {
            vga.delete_char();
            continue;
        }
        print!(ch);
    }
}

fn init_stack() {
    unsafe { 
        asm!("mov rbp, 0x10000201ff8",
             "mov ax, 0x10",
             "mov ss, ax"); 
    }

    let mut tmp_p = 0x40b0 as *mut u64;
    unsafe {
        *tmp_p.offset(0) = 0x16003;
        *tmp_p.offset(1) = 0x17003;
        *tmp_p.offset(2) = 0x18003;
        *tmp_p.offset(3) = 0x19003;
        *tmp_p.offset(4) = 0x1a003;
        *tmp_p.offset(5) = 0x1b003;
        *tmp_p.offset(6) = 0x1c003;
        *tmp_p.offset(7) = 0x1d003;
    }
}


fn init_interrupt() {
    let gdtr_base : *mut u16 = 0x16000 as *mut u16;
    let idtr_base : *mut u32 = hleos::interrupt::init_gdt_tss(gdtr_base);

    hleos::interrupt::init_idt(idtr_base);

    unsafe {
        hleos::asm::lgdt(gdtr_base as u64);
        hleos::asm::load_tss(0x28);
        hleos::asm::lidt(idtr_base as u64);
        hleos::asm::sgdt(0x16b00 as *mut u64);
        hleos::asm::store_tss(0x16b10 as *mut u64);
        hleos::asm::sidt(0x16b20 as *mut u64);
    }
    
    hleos::timer::k_init_pit(1193 / 2, true);
    hleos::interrupt::init_pic();
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let vga : &hleos::vga::VgaHandle = hleos::vga::get_vga_handle();
    vga.print_line(b"Panic_occured!!\n");
    loop {}
}

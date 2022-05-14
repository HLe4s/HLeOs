#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::arch::asm;
use core::panic::PanicInfo;
use core::mem::size_of;
use core::ptr;
mod etc;
mod hleos;
mod std;

static mut ready_queue : u64 = 0x0;
static mut running_thread : *mut hleos::thread::Thread = 0x0 as *mut hleos::thread::Thread;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // kmalloc init를 수행해야 하는데, 수행하지 않았다. 다음에 꼭 유의해서 수행해주길 바람.
    // ^^ bitmap을 0으로 초기화 해야함.
    println!("\n  Initializing Stack ............................................  [    ]\n");
    etc::wait_a_moment();
    init_stack();
    cursor_print!(69, 1, "OK");
    etc::wait_a_moment();
    //init_kmalloc();
    println!("  Initializing thread ...........................................  [    ]\n");
    etc::wait_a_moment();
    init_thread();
    cursor_print!(69, 3, "OK");
    etc::wait_a_moment();
    println!("  Initializing interrupt ........................................  [    ]\n");
    etc::wait_a_moment();
    init_interrupt();
    cursor_print!(69, 5, "OK");
    etc::wait_a_moment();
    
    unsafe { 
        hleos::thread::load_thread(running_thread as *mut hleos::thread::Thread);
    }
    loop {};
}

fn init_stack() {
    unsafe { 
        asm!("mov rbp, 0x10000201ff8",
             "mov ax, 0x10",
             "mov ss, ax"); 
    }

    // Setting IST, GDT, IDT address
    let mut tmp_p = 0x40b0 as *mut u64;
    unsafe {
        *tmp_p.offset(0) = 0x16003;
        *tmp_p.offset(1) = 0x279003;
        *tmp_p.offset(2) = 0x27a003;
        *tmp_p.offset(3) = 0x27b003;
        *tmp_p.offset(4) = 0x27c003;
        *tmp_p.offset(5) = 0x27d003;
        *tmp_p.offset(6) = 0x27e003;
        *tmp_p.offset(7) = 0x27f003;

        tmp_p = tmp_p.offset(8);
    }

    let pml4 = 0x1000 as *mut u64;
    let mut tmp = 0x1e000 as *mut u64;
    // Setting Kernel heap
    unsafe {
        *pml4.offset(1) = 0x280003;

        *tmp_p = 0x280003;
        *tmp_p.offset(1) = 0x281003;

        *tmp = 0x281003;
        tmp = 0x1f000 as *mut u64;
        *tmp = 0x600083;

        *tmp_p = 0x0;
        *tmp_p.offset(1) = 0x0;
    }
    hleos::asm::flush_tlb(0x1e000);
    hleos::asm::flush_tlb(0x1f000);
}

fn init_kmalloc() {
    let kmalloc_addr = 0x8000000000 as *mut u8;

    for i in 0..0x7f20 {
        unsafe {
            *kmalloc_addr.offset(i) = 0x0;
        }
    }
}

fn init_thread() {
    let ready_queue_ptr : *mut std::queue::Queue = hleos::kmalloc::kmalloc(size_of::<std::queue::Queue>() as u64);
    let ready_queue_buffer : *mut u8 = hleos::kmalloc::kmalloc(0x40);
    let ready_queue_stack = std::queue::Queue::new(ready_queue_buffer, 8, 7);

    unsafe {
        ptr::write(ready_queue_ptr, ready_queue_stack);
        ready_queue = ready_queue_ptr as u64;
        hleos::thread::ready_thread(hleos::thread::create_thread(hleos::thread::jobs::print_hello,
                                                                 hleos::kmalloc::stack_kmalloc(0xff0)));
        hleos::thread::ready_thread(hleos::thread::create_thread(hleos::thread::jobs::print_hi,
                                                                 hleos::kmalloc::stack_kmalloc(0xff0)));
        hleos::thread::ready_thread(hleos::thread::create_thread(hleos::thread::jobs::getch_main,
                                                                 hleos::kmalloc::stack_kmalloc(0xff0)));
        running_thread = hleos::thread::create_thread(hleos::thread::jobs::init, 
                                                      hleos::kmalloc::stack_kmalloc(0xff0));
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

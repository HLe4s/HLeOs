use super::super::asm::save_context;
use super::super::asm::load_context;
use super::super::thread::current_thread;
use super::super::thread::copy_thread;
use super::super::thread::ready_thread;
use super::super::thread::pop_thread;
use super::super::thread::load_thread;
use super::super::thread::Thread;
use super::super::super::println;
use super::super::super::print;
use super::super::super::cursor_print;
use super::super::super::std;
use super::super::vga::get_vga_handle;
use super::k_send_eoi_to_pic;
use super::PIC_IRQSTARTVECTOR;
use core::arch::asm;

const VECTOR_TIMER : u8 = 0x20;
const VECTOR_KEYBOARD : u8 = 0x21;

static mut timer_ms : u32 = 0;
static mut timer_sec : u64 = 0;

pub fn k_isr_timer(){
    unsafe { asm!("push rax"); };
    save_context();
    common_interrupt_handler(VECTOR_TIMER);
    load_context();
}

pub fn k_isr_keyboard(){
    unsafe { asm!("push rax"); };
    save_context();
    keyboard_handler(VECTOR_KEYBOARD);
    load_context();
}

pub fn k_isr_divide_error(){
    save_context();
    common_interrupt_handler(0x0);
    load_context();
}

pub fn k_isr_debug_exception(){
    save_context();
    common_interrupt_handler(0x1);
    load_context();
}

pub fn k_isr_nmi_interrupt(){
    save_context();
    common_interrupt_handler(0x2);
    load_context();
}

pub fn k_isr_breakpoint(){
    save_context();
    common_interrupt_handler(0x3);
    load_context();
}

pub fn k_isr_overflow(){
    save_context();
    common_interrupt_handler(0x4);
    load_context();
}

pub fn k_isr_bound_exceeded(){
    save_context();
    common_interrupt_handler(0x5);
    load_context();
}

pub fn k_isr_invalid_opcode(){
    save_context();
    common_interrupt_handler(0x6);
    load_context();
}

pub fn k_isr_device_not_available(){
    save_context();
    common_interrupt_handler(0x7);
    load_context();
}

pub fn k_isr_double_fault(){
    save_context();
    common_interrupt_handler(0x8);
    load_context();
}

pub fn k_isr_cso(){
    save_context();
    common_interrupt_handler(0x9);
    load_context();
}

pub fn k_isr_invalid_tss(){
    save_context();
    common_interrupt_handler(0xa);
    load_context();
}

pub fn k_isr_segment_not_present(){
    save_context();
    common_interrupt_handler(0xb);
    load_context();
}

pub fn k_isr_stack_segment_fault(){
    save_context();
    common_interrupt_handler(0xc);
    load_context();
}

pub fn k_isr_general_protection(){
    save_context();
    common_interrupt_handler(0xd);
    load_context();
}

pub fn k_isr_page_fault(){
    save_context();
    common_interrupt_handler(0xe);
    load_context();
}

pub fn k_isr_fpu_error(){
    save_context();
    common_interrupt_handler(0x10);
    load_context();
}

pub fn k_isr_alignment_check(){
    save_context();
    common_interrupt_handler(0x11);
    load_context();
}

pub fn k_isr_machine_check(){
    save_context();
    common_interrupt_handler(0x12);
    load_context();
}

pub fn k_isr_sfpe(){
    save_context();
    common_interrupt_handler(0x13);
    load_context();
}

pub fn k_isr_etc_interrupt() {
    unsafe { asm!("push rax"); }
    save_context();
    dummy_handler(255);
    load_context();
}

fn common_interrupt_handler(vector : u8){
    static mut common_interrupt_cnt : i32 = 0;
    let mut vga_handle = get_vga_handle();

    match vector
    {
        VECTOR_TIMER => {
            let mut rsp : u64 = 0x0;
            unsafe {
                if timer_ms >= 100 {
                    asm!("mov rax, rsp",
                        out("rax") rsp);
                    rsp += 0x70;

                    copy_thread(current_thread(), rsp as *mut Thread);
                    ready_thread(current_thread());

                    timer_ms = 0;
                    timer_sec += 1;
                    if timer_sec % 20 < 10 {
                        vga_handle.cursor_visible();
                    } else if vga_handle.is_cursor_visible() {
                        vga_handle.cursor_invisible();
                    }
                    
                    timer_ms += 1;
                    k_send_eoi_to_pic(vector - PIC_IRQSTARTVECTOR);

                    load_thread(pop_thread());
                }
                timer_ms += 1;
            }
            k_send_eoi_to_pic(vector - PIC_IRQSTARTVECTOR);
        }
        _ => {
        }
    }
}

fn keyboard_handler(vector : u8){
    println!("dummy!");
    k_send_eoi_to_pic(vector - PIC_IRQSTARTVECTOR);
}

fn dummy_handler(vector : u8) {
    cursor_print!(58, 0, "DUMMY_HANDLER_OCCURED!");
}

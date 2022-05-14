mod handler;

use super::asm::k_enable_interrupt;
use super::asm::k_disable_interrupt;
use super::asm::kInPortByte;
use super::asm::kOutPortByte;
use super::asm::k_read_rflags;

const PIC_MASTER_PORT1: u16 = 0x20;
const PIC_MASTER_PORT2: u16 = 0x21;
const PIC_SLAVE_PORT1: u16 = 0xA0;
const PIC_SLAVE_PORT2: u16 = 0xA1;
const PIC_IRQSTARTVECTOR: u8 = 0x20;

pub fn init_gdt_tss(gdtr : *mut u16) -> *mut u32 {
    let ist : [*mut u64; 8] = [ 0x17ff0 as *mut u64,
                                0x18ff0 as *mut u64,
                                0x19ff0 as *mut u64,
                                0x1aff0 as *mut u64,
                                0x1bff0 as *mut u64,
                                0x1cff0 as *mut u64,
                                0x1dff0 as *mut u64,
                                0x0 as *mut u64];
    unsafe {
        *gdtr = 0x38 - 1;
        let mut gdtr : *mut u32 = (gdtr.offset(0x1) as *mut u32);
        *gdtr = (gdtr as u32 + 0xe);
        gdtr = (gdtr as u32 + 0x6) as *mut u32;
        *gdtr = 0x0;
        gdtr = (gdtr as u32 + 0x8) as *mut u32;

        let mut gdtr = set_null_descriptor(gdtr);
        let mut gdtr = set_descriptor(gdtr, 0xA, true, 0);
        let mut gdtr = set_descriptor(gdtr, 0x2, true, 0);
        let mut gdtr = set_descriptor(gdtr, 0xA, true, 3);
        let mut gdtr = set_descriptor(gdtr, 0x2, true, 3);
        let mut gdtr = set_tss_descriptor(gdtr, unsafe{ gdtr.offset(0x4) as u64 }, 104 - 1, 0);
        let mut gdtr : u64 = set_tss(gdtr as *mut u32, &ist) as u64;
        let mut gdtr = gdtr + gdtr % 0x10;
        gdtr as *mut u32
    }
}

pub fn init_idt(pidt : *mut u32){
    unsafe {
        *pidt = 0x10 * 100 - 1;
        let mut pidt = (pidt as u64 + 2) as *mut u64;

        *pidt = (pidt as u64) + 0xe;
        let mut pidt = *pidt as *mut u32;

        pidt = set_idt_gate_descriptor(pidt, handler::k_isr_divide_error, 0x8, 1, 0, false);
        pidt = set_idt_gate_descriptor(pidt, handler::k_isr_debug_exception, 0x8, 2, 0, false);
        pidt = set_idt_gate_descriptor(pidt, handler::k_isr_nmi_interrupt, 0x8, 3, 0, false);
        pidt = set_idt_gate_descriptor(pidt, handler::k_isr_breakpoint, 0x8, 4, 0, false);
        pidt = set_idt_gate_descriptor(pidt, handler::k_isr_overflow, 0x8, 5, 0, false);
        pidt = set_idt_gate_descriptor(pidt, handler::k_isr_bound_exceeded, 0x8, 6, 0, false);
        pidt = set_idt_gate_descriptor(pidt, handler::k_isr_invalid_opcode, 0x8, 7, 0, false);
        pidt = set_idt_gate_descriptor(pidt, handler::k_isr_device_not_available, 0x8, 1, 0, false);
        pidt = set_idt_gate_descriptor(pidt, handler::k_isr_double_fault, 0x8, 2, 0, false);
        pidt = set_idt_gate_descriptor(pidt, handler::k_isr_cso, 0x8, 3, 0, false);
        pidt = set_idt_gate_descriptor(pidt, handler::k_isr_invalid_tss, 0x8, 4, 0, false);
        pidt = set_idt_gate_descriptor(pidt, handler::k_isr_segment_not_present, 0x8, 5, 0, false);
        pidt = set_idt_gate_descriptor(pidt, handler::k_isr_stack_segment_fault, 0x8, 6, 0, false);
        pidt = set_idt_gate_descriptor(pidt, handler::k_isr_general_protection, 0x8, 7, 0, false);
        pidt = set_idt_gate_descriptor(pidt, handler::k_isr_page_fault, 0x8, 1, 0, false);
        pidt = set_idt_gate_descriptor(pidt, handler::k_isr_etc_interrupt, 0x8, 2, 0, false);
        pidt = set_idt_gate_descriptor(pidt, handler::k_isr_fpu_error, 0x8, 3, 0, false);
        pidt = set_idt_gate_descriptor(pidt, handler::k_isr_alignment_check, 0x8, 4, 0, false);
        pidt = set_idt_gate_descriptor(pidt, handler::k_isr_machine_check, 0x8, 5, 0, false);
        pidt = set_idt_gate_descriptor(pidt, handler::k_isr_sfpe, 0x8, 6, 0, false);

        for i in 20..32 {
            pidt = set_idt_gate_descriptor(pidt, handler::k_isr_etc_interrupt, 0x8, i % 7 + 1, 0, false);
        }

        pidt = set_idt_gate_descriptor(pidt, handler::k_isr_timer, 0x8, 5, 0, false);
        pidt = set_idt_gate_descriptor(pidt, handler::k_isr_keyboard, 0x8, 6, 0, false);

        for i in 34..100 {
            pidt = set_idt_gate_descriptor(pidt, handler::k_isr_etc_interrupt, 0x8, i % 7 + 1, 0, false);
        }
    }
}

pub fn init_pic() {
    k_init_pic();
    k_mask_pic_interrupt(2);
}

fn set_descriptor(dptr : *mut u32, typ : u32, s : bool, dpl : u32) -> *mut u32 {
    unsafe {
        *dptr = 0x0;
        *dptr.offset(0x1) = 0x0;

        let typ : u32 = typ << 8;
        *dptr.offset(0x1) = typ;
        *dptr.offset(0x1) &= 0xf00;
        if s {
            *dptr.offset(0x1) |= 0x1000;
        }
        *dptr.offset(0x1) |= 0x8000;

        let dpl : u32 = dpl << 13;
        *dptr.offset(0x1) &= 0xffff9fff;
        *dptr.offset(0x1) |= dpl;

        *dptr.offset(0x1) &= 0xff0fffff;
        *dptr.offset(0x1) |= 0x00200000;

        *dptr = 0xffff;
        dptr.offset(0x2)
    }
}

fn set_tss_descriptor(dptr : *mut u32, addr : u64, size : u32, dpl : u32) -> *mut u32 {
    let addr_lo : u32 = (addr & 0xffffffff) as u32;
    let addr = (addr & 0xffffffff00000000) >> 32;
    let addr_hi : u32 = (addr & 0xffffffff) as u32;
    unsafe {
        *dptr.offset(0x1) = 0x0;
        *dptr.offset(0x2) = 0x0;
        *dptr.offset(0x3) = 0x0;

        set_descriptor(dptr, 0x9, false, dpl);
        *dptr = 0x0;

        *dptr |= size & 0xffff;
        *dptr |= (addr_lo & 0xffff) << 16;

        *dptr.offset(0x1) |= ((addr_lo & 0xff0000) >> 16);
        *dptr.offset(0x1) |= (addr_lo & 0xff000000);
        *dptr.offset(0x1) &= 0xff0fffff;
        *dptr.offset(0x1) |= 0x00800000;

        *dptr.offset(0x2) = (addr_hi);
        dptr.offset(0x4)
    }
}

fn set_null_descriptor(dptr : *mut u32) -> *mut u32 {
    unsafe {
        *dptr = 0x0;
        *dptr.offset(0x1) = 0x0;
        dptr.offset(0x2)
    }
}

fn set_tss(dptr : *mut u32, ist : &[*mut u64]) -> *mut u32 {
    let mut cnt : isize = 0;
    let mut dp : *mut u64;
    unsafe {
        *dptr.offset(0x0) = 0x0;
        *dptr.offset(0x1) = 0x0;
        *dptr.offset(0x2) = 0x0;
        *dptr.offset(0x3) = 0x0;
        *dptr.offset(0x4) = 0x0;
        *dptr.offset(0x5) = 0x0;
        *dptr.offset(0x6) = 0x0;
        *dptr.offset(0x7) = 0x0;
        *dptr.offset(0x8) = 0x0;
        *dptr.offset(0x9) = 0x0;
        *dptr.offset(0xa) = 0x0;
        *dptr.offset(0xb) = 0x0;
        *dptr.offset(0xc) = 0x0;
        *dptr.offset(0xd) = 0x0;
        *dptr.offset(0xe) = 0x0;
        *dptr.offset(0xf) = 0x0;
        *dptr.offset(0x10) = 0x0;
        *dptr.offset(0x11) = 0x0;
        *dptr.offset(0x12) = 0x0;
        *dptr.offset(0x13) = 0x0;
        *dptr.offset(0x14) = 0x0;
        *dptr.offset(0x15) = 0x0;
        *dptr.offset(0x16) = 0x0;

        dp = dptr.offset(0x9) as *mut u64;

        while ist[cnt as usize] as u64 != 0x0 {
            *dp.offset(cnt) = ist[cnt as usize] as u64;
            cnt += 1;
        }

        *dptr.offset(0x17) = 0x0;
        *dptr.offset(0x18) = 0x0;

        *dptr.offset(0x19) = 0xffff0000;
        dptr.offset(0x20)
    }
}

fn set_idt_gate_descriptor(pidt : *mut u32, handler : fn(), segment : u8, ist : u8, dpl : u8, trap : bool) -> *mut u32 {
    unsafe {
        *pidt = 0x0;
        *pidt.offset(0x1) = 0x0;
        *pidt.offset(0x2) = 0x0;
        *pidt.offset(0x3) = 0x0;

        *pidt = (handler as u64 & 0xffff) as u32;
        *pidt |= ((segment as u64) << 16) as u32;

        *pidt.offset(0x1) = (handler as u64 & 0xffff0000) as u32;
        *pidt.offset(0x1) |= (ist % 0x8) as u32;
        if trap {
            *pidt.offset(0x1) |= (0xf << 8) as u32;
        } else {
            *pidt.offset(0x1) |= (0xe << 8) as u32;
        }
        *pidt.offset(0x1) |= (((dpl % 0x4) as u32) << 0xd) as u32;
        *pidt.offset(0x1) |= 0x8000;

        *pidt.offset(0x2) = ((handler as u64 & 0xffffffff00000000) >> 32) as u32;

        pidt.offset(0x4)
    }
}

fn k_init_pic() {
    // ICW1 | LTIM = 0, SNGL = 0, IC4 = 1
    kOutPortByte(PIC_MASTER_PORT1, 0x11);
    // ICW2 | IRQ -> INTERRUPT VECTOR
    kOutPortByte(PIC_MASTER_PORT2, PIC_IRQSTARTVECTOR);
    // ICW3 | SLAVE PIC BIT = 0x4 -> master's pin to slave (bit-mask)
    kOutPortByte(PIC_MASTER_PORT2, 0x4);
    // ICW4 | uPM = 0x1
    kOutPortByte(PIC_MASTER_PORT2, 0x1);

    kOutPortByte(PIC_SLAVE_PORT1, 0x11);
    kOutPortByte(PIC_SLAVE_PORT2, PIC_IRQSTARTVECTOR + 8);
    // ICW3 | MASTER PIC BIT = 0x2 -> master's pin to slave (number)
    kOutPortByte(PIC_SLAVE_PORT2, 0x2);
    kOutPortByte(PIC_SLAVE_PORT2, 0x1);
}

fn k_mask_pic_interrupt(irq_bit_mask : u16) {
    kOutPortByte(PIC_MASTER_PORT2, (irq_bit_mask & 0xff) as u8);
    kOutPortByte(PIC_SLAVE_PORT2, ((irq_bit_mask & 0xff00) >> 8) as u8);
}

fn k_send_eoi_to_pic(irq_number : u8) {
    kOutPortByte(PIC_MASTER_PORT1, 0x20);
    if irq_number >= 8 {
        kOutPortByte(PIC_SLAVE_PORT1, 0x20);
    }
}

pub fn set_interrupt_flag(iF : bool) -> bool {
    let rflags = k_read_rflags();
    let old_if = if rflags & 0x200 == 0 {false} else {true};

    if iF {
        k_enable_interrupt();
    } else {
        k_disable_interrupt();
    }

    old_if
}

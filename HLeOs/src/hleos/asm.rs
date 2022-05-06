use core::arch::asm;

#[inline]
pub fn load_tss(sel : u32){
    unsafe {
        asm!("ltr {0:x}", in(reg) sel);
    }
}

#[inline]
pub fn store_tss(sel : *const u64){
    unsafe {
        asm!("str [{}]", in(reg) sel);
    }
}

#[inline]
pub fn lidt(idt : u64){
	unsafe {
        asm!("lidt [{}]", in(reg) idt);
    }
}

#[inline]
pub fn sidt(idt : *const u64){
	unsafe {
        asm!("sidt [{}]", in(reg) idt);
    }
}

#[inline]
pub fn lgdt(gdt : u64){
	unsafe {
        asm!("lgdt [{}]", in(reg) gdt);
    }
}

#[inline]
pub fn sgdt(gdt : *const u64){
	unsafe {
        asm!("sgdt [{}]", in(reg) gdt);
    }
}

#[inline]
pub fn flush_tlb(addr: u64) {
    unsafe {
        asm!("invlpg [{}]", in(reg) addr);
    }
}
pub fn kInPortByte(p : u16) -> u8 {
    let mut ret : u8 = 0;
    unsafe {
        asm!("in al, dx",
             out("al") ret,
             in("dx") p);
    }
    ret
}

pub fn kOutPortByte(p : u16, d : u8) {
    unsafe {
        asm!("out dx, al", 
             in("al") d, 
             in("dx") p);
    }
}

#[inline(always)]
pub fn save_context(){
    unsafe {
        asm!("push rbp",
             "mov rbp, rsp",
             "push rax",
             "push rbx",
             "push rcx",
             "push rdx",
             "push rdi",
             "push rsi",
             "push r8",
             "push r9",
             "push r10",
             "push r11",
             "push r12",
             "push r13",
             "push r14",
             "push r15",
             "mov ax, ds",
             "push rax",
             "mov ax, es",
             "push rax",
             "push fs",
             "push gs",
             "mov ax, 0x10",
             "mov ds, ax",
             "mov es, ax",
             "mov gs, ax",
             "mov fs, ax");
    }
}

#[inline(always)]
pub fn load_context(){
    unsafe {
        asm!("pop gs",
             "pop fs",
             "pop rax",
             "mov es, ax",
             "pop rax",
             "mov ds, ax",
             "pop r15",
             "pop r14",
             "pop r13",
             "pop r12",
             "pop r11",
             "pop r10",
             "pop r9",
             "pop r8",
             "pop rsi",
             "pop rdi",
             "pop rdx",
             "pop rcx",
             "pop rbx",
             "pop rax",
             "pop rbp",
             "add rsp, 0x10",
             "iretq");
    }
}

pub fn k_enable_interrupt(){
    unsafe {
        asm!("sti");
    }
}

pub fn k_disable_interrupt(){
    unsafe {
        asm!("cli");
    }
}

pub fn k_read_rflags() -> usize{
    let mut ret : usize = 0x0;
    unsafe {
        asm!("pushfq",
             "pop rax",
             out("rax") ret);
    }
    ret
}

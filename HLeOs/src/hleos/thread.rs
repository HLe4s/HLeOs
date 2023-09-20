use super::asm;
use core::arch::asm;
use super::super::ready_queue;
use super::super::running_thread;
use super::kmalloc;
use core::mem::size_of;
use core::ptr;
use super::super::std::queue;

pub mod jobs;

#[repr(C)]
pub struct Thread{
    gs : u64,
    fs : u64,
    es : u64,
    ds : u64,
    r15 : u64,
    r14 : u64,
    r13 : u64,
    r12 : u64,
    r11 : u64,
    r10 : u64,
    r9 : u64,
    r8 : u64,
    rsi : u64,
    rdi : u64,
    rdx : u64,
    rcx : u64,
    rbx : u64,
    rax : u64,
    rbp : u64,
    pad : u64,
    pad2 : u64,
    rip : u64,
    cs : u64,
    rflags : u64,
    rsp : u64,
    ss : u64,
    th_addr : u64,
}


pub fn get_addr(th : *mut Thread) -> *mut Thread {
    unsafe {
        ptr::read(th).th_addr as *mut Thread
    }
}

pub fn load_thread(th : *mut Thread) {
    unsafe {
        running_thread = th;
        asm!("mov rsp, rax",
             in("rax") th);
		asm!("pop rax",
			 "mov gs, ax",
			 "pop rax",
             "mov fs, ax",
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

pub fn save_thread(th : *mut Thread) -> *mut Thread {
    let stack_align = 0x228;
    let mut rip_val : u64 = 0x0;
    let mut rsp_val : u64 = 0x0;
    let mut rbp : u64 = 0x0;
    let mut rax : u64 = 0x0;
    let mut rbx : u64 = 0x0;
    let mut rcx : u64 = 0x0;
    let mut rdx : u64 = 0x0;
    let mut rdi : u64 = 0x0;
    let mut rsi : u64 = 0x0;
    let mut r8 : u64 = 0x0;
    let mut r9 : u64 = 0x0;
    let mut r10 : u64 = 0x0;
    let mut r11 : u64 = 0x0;
    let mut r12 : u64 = 0x0;
    let mut r13 : u64 = 0x0;
    let mut r14 : u64 = 0x0;
    let mut r15 : u64 = 0x0;
    let mut r15 : u64 = 0x0;
    let mut cs : u16 = 0x0;
    let mut ds : u16 = 0x0;
    let mut es : u16 = 0x0;
    let mut fs : u16 = 0x0;
    let mut gs : u16 = 0x0;
    let mut ss : u16 = 0x0;
    let mut rflags : u64 = 0x0;
    unsafe {
        asm!("", out("rax") rax);
        asm!("mov rax, rbx", out("rax") rbx);
        asm!("", out("rcx") rcx);
        asm!("", out("rdx") rdx);
        asm!("", out("rdi") rdi);
        asm!("", out("rsi") rsi);
        asm!("", out("r8") r8);
        asm!("", out("r9") r9);
        asm!("", out("r10") r10);
        asm!("", out("r11") r11);
        asm!("", out("r12") r12);
        asm!("", out("r13") r13);
        asm!("", out("r14") r14);
        asm!("xor rax, rax", out("r15") r15);
        asm!("mov ax, ss", out("ax") ss);
        asm!("mov ax, cs", out("ax") cs);
        asm!("mov ax, ds", out("ax") ds);
        asm!("mov ax, es", out("ax") es);
        asm!("mov ax, fs", out("ax") fs);
        asm!("mov ax, gs", out("ax") gs);
        asm!("mov rax, rbp", out("rax") rbp);
        asm!("pushfq",
             "pop rax",
             out("rax") rflags);
        // acquire rip, rsp value
        asm!("mov r8, [rsp + r9]",
             "mov r10, rsp",
             out("r8") rip_val,
             in("r9") stack_align,
             out("r10") rsp_val
            );
        rsp_val += stack_align + 0x8;
		ptr::write(th,
            Thread{
                ss : ss as u64,
                rsp : rsp_val,
                rflags : rflags,
                cs : cs as u64,
                rip : rip_val,
                pad : 0x0,
                pad2 : 0x0,
                rbp : rbp,
                rax : rax,
                rbx : rbx,
                rcx : rcx,
                rdx : rdx,
                rdi : rdi,
                rsi : rsi,
                r8 : r8,
                r9 : r9,
                r10 : r10,
                r11 : r11,
                r12 : r12,
                r13 : r13,
                r14 : r14,
                r15 : r15,
                ds : ds as u64,
                es : es as u64,
                fs : fs as u64,
                gs : gs as u64,
                th_addr : th as u64,
            });
    }
    th
}

pub fn create_thread(entry_point : fn(), stack : *mut u64) -> *mut Thread {
    let th = kmalloc::kmalloc(size_of::<Thread>() as u64);
    unsafe {
        ptr::write(th, 
            Thread{
                ss : 0x10,
                rsp : stack as u64,
                rflags : 0x0,
                cs : 0x8,
                rip : entry_point as u64,
                pad : 0x0,
                pad2 : 0x0,
				rbp : stack as u64,
                rax : 0x0,
                rbx : 0x0,
                rcx : 0x0,
                rdx : 0x0,
                rdi : 0x0,
                rsi : 0x0,
                r8 : 0x0,
                r9 : 0x0,
                r10 : 0x0,
                r11 : 0x0,
                r12 : 0x0,
                r13 : 0x0,
                r14 : 0x0,
                r15 : 0x0,
                ds : 0x10,
                es : 0x10,
                fs : 0x0,
                gs : 0x0,
                th_addr : th as u64,
            });
    }
    th
}

pub fn create_thread_c(entry_point : unsafe extern "C" fn(), stack : *mut u64) -> *mut Thread {
    let th = kmalloc::kmalloc(size_of::<Thread>() as u64);
    unsafe {
        ptr::write(th, 
            Thread{
                ss : 0x10,
                rsp : stack as u64,
                rflags : 0x0,
                cs : 0x8,
                rip : entry_point as u64,
                pad : 0x0,
                pad2 : 0x0,
				rbp : stack as u64,
                rax : 0x0,
                rbx : 0x0,
                rcx : 0x0,
                rdx : 0x0,
                rdi : 0x0,
                rsi : 0x0,
                r8 : 0x0,
                r9 : 0x0,
                r10 : 0x0,
                r11 : 0x0,
                r12 : 0x0,
                r13 : 0x0,
                r14 : 0x0,
                r15 : 0x0,
                ds : 0x10,
                es : 0x10,
                fs : 0x0,
                gs : 0x0,
                th_addr : th as u64,
            });
    }
    th
}

pub fn copy_thread(dst : *mut Thread, src : *mut Thread) -> *mut Thread {
	unsafe {
        let mut tmp = ptr::read(src);
        tmp.th_addr = dst as u64;
		ptr::write(dst, tmp);
	}
	dst
}

pub fn current_thread() -> *mut Thread {
    unsafe {
        running_thread
    }
}

pub fn ready_thread(th : *mut Thread) -> bool {
    let mut rq = unsafe { ptr::read(ready_queue as *mut queue::Queue) };
    if rq.enqueue(th as u64) {
        unsafe {
            ptr::write(ready_queue as *mut queue::Queue,
                       rq);
        }
        return true;
    }
    false
}

pub fn pop_thread() -> *mut Thread {
	let mut rq = unsafe { ptr::read(ready_queue as *mut queue::Queue) };
	let ret = rq.dequeue();
	if ret != 0xdeadbeafcafebabe {
		unsafe {
			ptr::write(ready_queue as *mut queue::Queue,
					   rq);
		}
		return ret as *mut Thread;
	} else {
		return 0x0 as *mut Thread;
	}
}

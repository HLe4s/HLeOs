use crate::hleos::vga;

pub trait PrintLn {
    fn print(&self);
}

impl PrintLn for i32 {
    fn print(&self) {
        let vga : &vga::VgaHandle = vga::get_vga_handle();
        vga.print_number(*self);
    }
}

impl PrintLn for isize {
    fn print(&self) {
        let vga : &vga::VgaHandle = vga::get_vga_handle();
        vga.print_number_i64(*self as i64);
    }
}

impl PrintLn for u64 {
    fn print(&self) {
        let vga : &vga::VgaHandle = vga::get_vga_handle();
        vga.print_number_u64(*self);
    }
}

impl PrintLn for i64 {
    fn print(&self) {
        let vga : &vga::VgaHandle = vga::get_vga_handle();
        vga.print_number_i64(*self);
    }
}

impl PrintLn for char {
    fn print(&self) {
        let vga : &vga::VgaHandle = vga::get_vga_handle();
        vga.print_char(*self as u8);
    }
}

impl PrintLn for u32 {
    fn print(&self) {
        let vga : &vga::VgaHandle = vga::get_vga_handle();
        vga.print_number(*self as i32);
    }
}

impl PrintLn for bool {
    fn print(&self) {
        let vga : &vga::VgaHandle = vga::get_vga_handle();
        if *self {
			vga.print_str("true");
		}
		else {
			vga.print_str("false");
		}	
    }
}

impl PrintLn for u8 {
    fn print(&self) {
        let vga : &vga::VgaHandle = vga::get_vga_handle();
        vga.print_char(*self);
    }
}

impl PrintLn for &'static str {
    fn print(&self) {
        let vga : &vga::VgaHandle = vga::get_vga_handle();
        vga.print_str(self);
    }
}

pub fn vs_println(input : &dyn PrintLn) {
    input.print();
}

pub fn next_line() {
    let vga : &vga::VgaHandle = vga::get_vga_handle();
    vga.print_char(b'\n');
}

pub fn clear() {
    let vga : &vga::VgaHandle = vga::get_vga_handle();
    vga.clear();
}

pub fn delete_a_char() {
    let vga : &vga::VgaHandle = vga::get_vga_handle();
    vga.delete_char();
}

pub fn set_cursor(x:u8, y:u8) {
    let vga : &vga::VgaHandle = vga::get_vga_handle();
    vga.set_cursor(x, y);
}

pub fn get_cursor() -> (u8, u8) {
    let vga : &vga::VgaHandle = vga::get_vga_handle();
    vga.get_cursor()
}

#[macro_export]
macro_rules! println {
    () => {
        crate::std::io::next_line();
    };
    ($($arg:expr), *)=> {
        $(
            crate::std::io::vs_println(&$arg);
        )*
        crate::std::io::next_line();
    };
}
#[macro_export]
macro_rules! print {
    ($($arg:expr), *)=> {
        $(
            crate::std::io::vs_println(&$arg);
        )*
    };
}
#[macro_export]
macro_rules! cursor_print {
    ($x:literal, $y:literal, $($arg:expr), *)=> {
		let (x, y) = crate::std::io::get_cursor();
		crate::std::io::set_cursor($x, $y);
        $(
            crate::std::io::vs_println(&$arg);
        )*
		crate::std::io::set_cursor(x, y);
    };
	($x:expr, $y:expr, $($arg:expr), *)=> {
        let (x, y) = crate::std::io::get_cursor();
        crate::std::io::set_cursor($x, $y);
        $(
            crate::std::io::vs_println(&$arg);
        )*
        crate::std::io::set_cursor(x, y);
    };
}

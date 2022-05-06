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

impl PrintLn for u32 {
    fn print(&self) {
        let vga : &vga::VgaHandle = vga::get_vga_handle();
        vga.print_number(*self as i32);
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
        std::io::next_line();
    };
    ($($arg:expr), *)=> {
        $(
            crate::std::io::vs_println(&$arg);
        )*
        std::io::next_line();
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
		let (x, y) = std::io::get_cursor();
		std::io::set_cursor($x, $y);
        $(
            crate::std::io::vs_println(&$arg);
        )*
		std::io::set_cursor(x, y);
    };
}

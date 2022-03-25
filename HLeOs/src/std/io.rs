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

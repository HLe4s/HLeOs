pub struct VgaHandle{
    x : u8,
    y : u8,
    def_attr : u8,
    cur_attr : u8,
}

impl VgaHandle {
    pub fn move_right_cursor(&mut self) -> bool {
        if self.x + 1 >= 0 && self.x + 1 < 80 {
            self.x = self.x + 1;
            true
        } else {
            false
        }
    }
    fn valid_cursor(&self) -> bool {
        if self.x >= 0 && self.x < 80 {
            if self.y >= 0 && self.y < 25 {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn print_char(&self, ch : u8) {
        let mut vga_buffer = 0xb8000 as *mut u8;

        if self.valid_cursor() {
            unsafe {
                vga_buffer = vga_buffer.offset(self.x as isize * 2 + self.y as isize * 160);

                *vga_buffer = ch;
                *vga_buffer.offset(1 as isize) = self.cur_attr;
        
                vga_handle.move_right_cursor();
            }
        }
    }

    pub fn clear(&self) {
		let vga_buffer = 0xb8000 as *mut u8;

    	for x in 0..80 {
			for y in 0..25 {
	        	unsafe {
		           	*vga_buffer.offset(x as isize * 2 + y as isize * 160) = 0x00;
			        *vga_buffer.offset(x as isize * 2 + 1 + y as isize * 160) = 0x00;
			    }
            }
		}
    }
}

static mut vga_handle : VgaHandle = VgaHandle {
    x : 0,
    y : 0,
    def_attr : 0x7,
    cur_attr : 0x7,
};

pub fn get_vga_handle() -> &'static VgaHandle {
    unsafe {
        &vga_handle
    }
}


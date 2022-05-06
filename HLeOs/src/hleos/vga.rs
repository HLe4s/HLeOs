pub struct VgaHandle{
    x : u8,
    y : u8,
    def_attr : u8,
    cur_attr : u8,
    cursor : bool,
}

impl VgaHandle {
    pub fn is_cursor_visible(&self) -> bool {
        self.cursor
    }
    pub fn cursor_visible(&self) {
        unsafe {
            vga_handle.print_char('|' as u8);
            vga_handle.cursor = true;
        }
    }
    pub fn get_cursor(&self) -> (u8, u8) {
        return (self.x, self.y);
    }
    pub fn set_cursor(&self, x : u8, y : u8){
        unsafe {
            vga_handle.x = x;
            vga_handle.y = y;
        }
    }
    pub fn cursor_print_line(&self, x : u8, y : u8, s : &[u8]){
        let (old_x, old_y) = self.get_cursor();
        self.set_cursor(x, y);
        self.print_line(s);
        self.set_cursor(old_x, old_y);
    }
    pub fn cursor_invisible(&self) {
        unsafe {
            vga_handle.delete_char();
            vga_handle.cursor = false;
        }
    }
    pub fn move_right_cursor(&mut self) -> bool {
        if self.x + 1 >= 0 && self.x + 1 < 80 {
            self.x = self.x + 1;
            true
        } else if self.x + 1 >= 80 {
            if self.y + 1 >= 0 && self.y + 1 < 25 {
                self.y = self.y + 1;
                self.x = 0;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn move_left_cursor(&mut self) -> bool {
        if self.x > 0 {
            self.x = self.x - 1;
            true
        } else {
            if self.y > 0 {
                let mut vga_buffer = 0xb8000 as *mut u8;
                self.y = self.y - 1;
                for i in (0..80).rev() {
                    unsafe {
                        if *vga_buffer.offset(i as isize * 2 + self.y as isize * 160) != 0x00 || i == 0 {
                            self.x = i;
                            break;
                        }
                    }
                }
                true
            } else {
                false
            }
        }
    }

    pub fn move_down_cursor(&mut self) -> bool {
        if self.y + 1 >= 0 && self.y + 1 < 25 {
            self.y = self.y + 1;
            true
        } else {
            false
        }
    }

    pub fn move_up_cursor(&mut self) -> bool {
        if self.y > 0 {
            self.y = self.y - 1;
            true
        } else {
            false
        }
    }

    pub fn next_line(&mut self) -> bool {
        if self.move_down_cursor() {
            self.x = 0;
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

        if self.cursor {
            unsafe {vga_handle.cursor_invisible()};
        }

        if self.valid_cursor() {
            if ch == b'\n' {
                unsafe {
                    vga_handle.next_line();
                }
            } else {
                unsafe {
                    vga_buffer = vga_buffer.offset(self.x as isize * 2 + self.y as isize * 160);

                    *vga_buffer = ch;
                    *vga_buffer.offset(1 as isize) = self.cur_attr;
        
                    vga_handle.move_right_cursor();
                }
            }
        }
    }

    pub fn print_number(&self, num : i32) {
        let mut n = num;
        if n < 0 {
            self.print_char(b'-');
            n *= -1;
        }

        if n / 10 != 0 {
            self.print_number(n / 10);
        }
        self.print_char(b'0' + (n % 10) as u8);
    }

    pub fn print_line(&self, s : &[u8]) {
       if self.valid_cursor() {
			for (i, &byte) in s.iter().enumerate() {
				self.print_char(byte);
		    }	
        } 
    }

    pub fn print_str(&self, s : &str) {
		if self.valid_cursor() {
			self.print_line(s.as_bytes());
        }
    }

    pub fn delete_line(&self) {
        loop {
            if self.x == 0 {
                break;
            }
            self.delete_char();
        }
    }

	pub fn delete_char(&self) {
        let mut vga_buffer = 0xb8000 as *mut u8;

        if self.valid_cursor() {
            unsafe {
                if vga_handle.move_left_cursor() {
                    vga_buffer = vga_buffer.offset(self.x as isize * 2 + self.y as isize * 160);
    
                    *vga_buffer = 0x00;
                    *vga_buffer.offset(1 as isize) = 0x00;
                }
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
        unsafe {
            vga_handle.x = 0;
            vga_handle.y = 0;
        }
    }
}

static mut vga_handle : VgaHandle = VgaHandle {
    x : 0,
    y : 0,
    def_attr : 0x7,
    cur_attr : 0x7,
    cursor : true,
};

pub fn get_vga_handle() -> &'static VgaHandle {
    unsafe {
        &vga_handle
    }
}


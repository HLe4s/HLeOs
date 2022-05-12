pub struct BitMap{
    bits : *mut u64,
    len : u64,
}

impl BitMap {
    pub fn new(ptr : *mut u64, l : u64) -> BitMap {
        BitMap {
            bits : ptr,
            len : l,
        }
    }
    pub fn len(&self) -> u64 {
        return self.len;
    }
    pub fn set_bit(&self, offset : u64) -> bool {
        if self.len <= offset {
            return false;
        }

        let mut qword_offset : u64 = offset / 64;
        let mut bit_offset : u64 = offset % 64;
        let qword : *mut u64 = unsafe { self.bits.offset(qword_offset as isize) };
        unsafe {
            *qword |= 1 << bit_offset;
        };

        true
    }
    pub fn unset_bit(&self, offset : u64) -> bool {
        if self.len <= offset {
            return false;
        }

        let mut qword_offset : u64 = offset / 64;
        let mut bit_offset : u64 = offset % 64;
        let qword : *mut u64 = unsafe { self.bits.offset(qword_offset as isize) };
        unsafe {
            *qword &= !(1 << bit_offset);
        };

        true
    }
    pub fn is_set(&self, offset : u64) -> bool {
		if self.len <= offset {
            return false;
        }

        let mut qword_offset : u64 = offset / 64;
        let mut bit_offset : u64 = offset % 64;
        let qword : *mut u64 = unsafe { self.bits.offset(qword_offset as isize) };
        unsafe {
			if *qword & (1 << bit_offset) != 0 {
				return true;
			} else {
				return false;
			}
        };
    }
    pub fn find_first(&self, tf:bool) -> u64 {
        let mut i : u64 = 0x0;
        while i < self.len {
            if self.is_set(i) == tf {
                return i;
            }
            i += 1;
        }
        return self.len;
    }
}

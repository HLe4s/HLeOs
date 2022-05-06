use core::panic;

pub struct Queue {
    q_buffer : *mut u8,
    data_unit_size : u8,
    max_len : u64,
    head : u64,
    tail : u64,
}

impl Queue {
    pub fn new(ptr : *mut u8, unit_size : u8, max : u64) -> Queue {
        if unit_size > 8 {
            panic!("Queue unit size can't be over 8");
        }
        Queue { q_buffer : ptr,
        data_unit_size : unit_size,
        max_len : max + 1,
        head : 0,
        tail : 0}
    }

    pub fn enqueue(&mut self, data : u64) -> bool {
        if self.is_full() {
            return false;
        }

        unsafe {
            let mut ptr : *mut u8 = self.q_buffer.offset((self.tail * (self.data_unit_size as u64)) as isize);
            let mut i : isize = 0;

            while i < self.data_unit_size as isize {
                *ptr.offset(i) = ((data >> 8 * i) & 0xff) as u8;
                i += 1;
            }

            self.tail = (self.tail + 1) % self.max_len;
            true
        }
    }
    pub fn dequeue(&mut self) -> u64 {
        let mut ret : u64 = 0x0;
        let mut i : isize = 0x0;
        let mut tmp : u64 = 0x0;

        if self.is_empty() {
            return 0xdeadbeafcafebabe;
        }

        unsafe {
            let ptr : *mut u8 = self.q_buffer.offset((self.head * (self.data_unit_size as u64)) as isize);
            while i < self.data_unit_size as isize {
                tmp = *ptr.offset(i) as u64;
                ret |= tmp << 8 * i;
                i += 1;
            }

            self.head = (self.head + 1) % self.max_len;
            ret
        }
    }
    pub fn is_empty(&self) -> bool {
        if self.head == self.tail {
            true
        } else {
            false
        }
    }
    pub fn is_full(&self) -> bool {
        if (self.tail + 1) % self.max_len == self.head {
            true
        } else {
            false
        }
    } 
}

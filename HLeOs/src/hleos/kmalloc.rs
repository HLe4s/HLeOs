use super::super::std::bit_map::BitMap;
use super::super::std;
use super::super::println;

pub fn kmalloc<T>(size : u64) -> *mut T {
    if size == 0 {
        return 0x0 as *mut T;
    }
    let heap_base = 0x8000000000 as *mut u64;
    let heap_mem  = (heap_base as u64 + 0x3f90) as *mut u64;
    let hbit_map = BitMap::new(heap_base, 130055);
    let size = (size + 0xf) / 0x10;

    let chk = find_continuous_false(&hbit_map, size);

    if chk != hbit_map.len() {
        let mut i : u64 = 0x0;
        while i < size {
            hbit_map.set_bit(chk + i);
            i += 1;
        }
        return (heap_mem as u64 + chk * 0x10) as *mut T;
    } else {
        return 0x0 as *mut T;
    }
}

pub fn stack_kmalloc<T>(size : u64) -> *mut T {
    if size == 0 {
        return 0x0 as *mut T;
    }
    let heap_base = 0x8000000000 as *mut u64;
    let heap_mem  = (heap_base as u64 + 0x3f90) as *mut u64;
    let hbit_map = BitMap::new(heap_base, 130055);
    let size = (size + 0xf) / 0x10;

    let chk = find_continuous_false(&hbit_map, size);

    if chk != hbit_map.len() {
        let mut i : u64 = 0x0;
        while i < size {
            hbit_map.set_bit(chk + i);
            i += 1;
        }
        return (heap_mem as u64 + chk * 0x10 + (size - 1) * 0x10) as *mut T;
    } else {
        return 0x0 as *mut T;
    }
}

pub fn free<T>(chk : *mut T, size : u64) -> bool {
    let heap_base = 0x8000000000 as *mut u64;
    let heap_mem  = (heap_base as u64 + 0x3f90) as *mut u64;
    let hbit_map = BitMap::new(heap_base, 130055);

    let size = (size + 0xf) / 0x10;
    let mut i : u64 = 0x0;

    let chk : u64 = (chk as u64 - heap_mem as u64) / 0x10;

    while i < size {
        hbit_map.unset_bit(chk + i);
        i += 1;
    }
    true
}

pub fn find_continuous_false(hbit_map : &BitMap, num : u64) -> u64 {
    let mut i : u64 = 0x0;
    let mut tmp : u64 = num;
    let mut first : u64 = hbit_map.len();

    while i < hbit_map.len() {
        if hbit_map.is_set(i) == false {
            if tmp == num {
                first = i;
            }
            tmp -= 1;
        } else {
            tmp = num;
        }
        if tmp == 0 {
            return first; 
        }
        i += 1;
    }
    return hbit_map.len();
}

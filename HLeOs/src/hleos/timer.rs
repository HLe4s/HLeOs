use super::asm::kInPortByte;
use super::asm::kOutPortByte;
use super::interrupt::set_interrupt_flag;

const PIT_FREQUENCY : u32 = 1193180;
const PIT_PORT_CONTROL : u16 = 0x43;
const PIT_PORT_COUNTER0 : u16 = 0x40;

const PIT_CONTROL_COUNTER0 : u8 = 0x00;
const PIT_CONTROL_LSBMSBRW : u8 = 0x30;
const PIT_CONTROL_MODE0 : u8 = 0x00;
const PIT_CONTROL_MODE2 : u8 = 0x04;
const PIT_CONTROL_BINARYCOUNTER : u8 = 0x00;
const PIT_CONTROL_BCDCOUNTER : u8 = 0x01;

const PIT_COUNTER0_LATCH : u8 = 0x00;
const PIT_COUNTER0_ONCE : u8 = PIT_CONTROL_COUNTER0 | PIT_CONTROL_LSBMSBRW | PIT_CONTROL_MODE0 | PIT_CONTROL_BINARYCOUNTER;

const PIT_COUNTER0_PERIODIC : u8 = PIT_CONTROL_COUNTER0 | PIT_CONTROL_LSBMSBRW | PIT_CONTROL_MODE2 | PIT_CONTROL_BINARYCOUNTER;

fn ms_to_count(x : u32) -> u32 {
    return (PIT_FREQUENCY * x) / 1000;
}

fn us_to_count(x : u32) -> u32 {
    return (PIT_FREQUENCY * x) / 1000000;
}

pub fn k_init_pit(cnt : u16, periodic : bool) {

    if periodic {
        kOutPortByte( PIT_PORT_CONTROL, PIT_COUNTER0_PERIODIC);
    } else {
        kOutPortByte( PIT_PORT_CONTROL, PIT_COUNTER0_ONCE);
    }

    kOutPortByte(PIT_PORT_COUNTER0, cnt as u8);
    kOutPortByte(PIT_PORT_COUNTER0, (cnt >> 8) as u8);
}

pub fn k_read_counter0() -> u16 {
    let mut low : u8 = 0x0;
    let mut high : u8 = 0x0;
    let mut cnt : u16 = 0x0;
    kOutPortByte(PIT_PORT_CONTROL, PIT_COUNTER0_LATCH);

    low = kInPortByte(PIT_PORT_COUNTER0);
    high = kInPortByte(PIT_PORT_COUNTER0);

    cnt = low as u16;
    cnt |= ((high as u16) << 8);

    cnt
}

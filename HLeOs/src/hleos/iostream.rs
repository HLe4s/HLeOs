use super::asm::kInPortByte;
use super::asm::kOutPortByte;

struct kKeyMappingEntryStruct {
    bNormalCode : u8,
    bCombinedCode : u8,
}

struct kKeyboardManager {
    bShiftDown : bool,
    bCapsLockOn : bool,
    bNumLockOn : bool,
    bScrollLockOn : bool,

    bExtendedCodeIn : bool,
    iSkipCountForPause : u8,
}

pub fn kIsOutputBufferFull() -> bool {
    let state : u8 = kInPortByte(0x64);
    
    if (state & 0x1) != 0 {
        true
    } else {
        false
    }
}

pub fn kIsInputBufferFull() -> bool {
    let state : u8 = kInPortByte(0x64);
    
    if (state & 0x2) != 0 {
        true
    } else {
        false
    }
}

pub fn kPutInputBuffer(ch : u8) {
    let mut cnt = 0;
    while kIsInputBufferFull() {
        if cnt == 0xffff {
            return;
        }
        cnt+=1;
    };
    kOutPortByte(0x60, ch);
}

pub fn kGetKeyboardScanCode() -> u8 {
    let mut cnt = 0;
    while !kIsOutputBufferFull() {
        if cnt == 0xffff {
            return 0x0;
        }
        cnt+=1;
    };
    kInPortByte(0x60)
}

pub fn kChangeKeyboardLED(capslock : bool,numlock : bool,scrollock : bool){
    let mut cnt = 0;
    let mut led_bit : u8 = 0;

    kPutInputBuffer(0xED);
    while kGetKeyboardScanCode() != 0xFA {
        if cnt == 0xff {
            return;
        }
        cnt+=1;
    }

    if capslock {
        led_bit |= 0x4;
    }
    if numlock {
        led_bit |= 0x2;
    }
    if scrollock {
        led_bit |= 0x1;
    }

    kPutInputBuffer(led_bit);
}

pub fn kActivateKeyboard() -> bool {
    let mut res : u8 = 0;
    let mut cnt : u8 = 0;

    kOutPortByte(0x64, 0xAE); // activate command to keyboard controller
    while kIsInputBufferFull() {
        if cnt == 0xff {
            cnt = 0;
            break;
        }
        cnt+=1;
    };
    kOutPortByte(0x60, 0xF4);

    
    while !kIsOutputBufferFull() {
        if cnt == 0xff {
            break;
        }
        cnt+=1;
    };
    res = kInPortByte(0x60);

    if res == 0xFA {
        true
    } else {
        false
    }
}

pub fn getch() -> u8{
    let mut key : u8 = 0;

    while !((key >= 0x10 && key <= 0x32) || key == 0x39 || key == 0x0E) {
        key = kGetKeyboardScanCode();
    }

    match key {
        0x0E => b'*',
        0x10 => b'Q',
        0x11 => b'W',
        0x12 => b'E',
        0x13 => b'R',
        0x14 => b'T',
        0x15 => b'Y',
        0x16 => b'U',
        0x17 => b'I',
        0x18 => b'O',
        0x19 => b'P',
        0x1c => b'\n',
        0x1e => b'A',
        0x1f => b'S',
        0x20 => b'D',
        0x21 => b'F',
        0x22 => b'G',
        0x23 => b'H',
        0x24 => b'J',
        0x25 => b'K',
        0x26 => b'L',
        0x27 => b';',
        0x28 => b'\'',
        0x2C => b'Z',
        0x2D => b'X',
        0x2E => b'C',
        0x2F => b'V',
        0x30 => b'B',
        0x31 => b'N',
        0x32 => b'M',
        0x39 => b' ',
        _ => b'/',
    }
}

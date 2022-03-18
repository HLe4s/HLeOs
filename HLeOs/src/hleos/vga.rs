pub struct VgaHandle{
    x : u8,
    y : u8,
    def_attr : u8,
    cur_attr : u8,
}

//impl VgaHandle{
//}

static vga_handle = VgaHandle {
    x : 0,
    y : 0,
    def_attr : 0x7,
    cur_attr : 0x7,
};

pub fn get_vga_handle() -> &' VgaHandle {
    &vga_handle;
}

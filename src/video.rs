use crate::ports::port_byte_out;

// #[repr(transparent)]
// struct Buffer {
//     chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
// }
//

// const ST00_READ_ADDRESS: u16 = 0x3C2;
// const ST01_READ_CGA_ADDRESS: u16 = 0x3DA;
// const ST01_READ_MDA_ADDRESS: u16 = 0x3BA;
// const FCR_READ_ADDRESS: u16 = 0x3CA;
// const FCR_CGA_WRITE_ADDRESS: u16 = 0x3DA;
// const FCR_MDA_WRITE_ADDRESS: u16 = 0x3BA;
// const MSR_READ_ADDRESS: u16 = 0x3CC;
// const MSR_WRITE_ADDRESS: u16 = 0x3C2;
//
// const SRX_INDEX_ADDRESS: u16 = 0x3C4;
// const SRX_DATA_ADDRESS: u16 = 0x3C5;
//
// const GRX_INDEX_ADDRESS: u16 = 0x3CE;
// const GRX_DATA_ADDRESS: u16 = 0x3CF;
//
// const ARX_INDEX_ADDRESS: u16 = 0x3C0;
// const ARX_DATA_ADDRESS: u16 = 0x3C1;
//
// const CRX_INDEX_CGA_ADDRESS: u16 = 0x3D4;
// const CRX_INDEX_MDA_ADDRESS: u16 = 0x3B4;
// const CRX_DATA_CGA_ADDRESS: u16 = 0x3D5;
// const CRX_DATA_MDA_ADDRESS: u16 = 0x3B5;
//
// const COLOR_PALETTE_DATA_ADDRESS: u16 = 0x3C9;
// const COLOR_PALETTE_INDEX_READ_ADDRESS: u16 = 0x3C7;
// const COLOR_PALETTE_INDEX_WRITE_ADDRESSS: u16 = 0x3C8;

fn write_sequencer(index: u8, value: u8) {
    port_byte_out(0x3C4, index);
    port_byte_out(0x3C5, value);
}

pub fn video_tmp() {
    // let mode = Graphics640x480x16::new();
    // mode.set_mode();
    // mode.clear_screen(Color16::Black);
    // mode.draw_line((80,60), (80,420), Color16::Blue);
    port_byte_out(0x3ce, 8);
    // port_byte_out(0x3cf, 0xff);
    port_byte_out(0x3ce, 5);
    port_byte_out(0x3cf, 2);

    write_sequencer(0x0, 0x03);
    write_sequencer(0x1, 0x01);
    write_sequencer(0x2, 0x08);
    write_sequencer(0x3, 0x00);
    write_sequencer(0x4, 0x06);

    // set mode:
    // set registers
    // msr read addr: 0x3cc
    // msr write addr: 0x3c2
    //
    // https://www.singlix.com/trdos/archive/vga/Graphics%20in%20pmode.pdf
    // https://forum.osdev.org/viewtopic.php?f=1&t=20137&hilit=640x480x16

    let video_buffer = 0xa0000 as *mut u8;
    for i in 0..(640 / 8 * 479) {
        unsafe { *video_buffer.offset(i) = 0x9 };
    }
    // port_byte_out(0x3ce, 0x5); // request read mode
    // println!("mode {:b}", port_byte_in(0x3cf));
    // port_byte_out(0x3ce, 0x5);
}

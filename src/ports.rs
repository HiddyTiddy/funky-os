use core::arch::asm;

use crate::println;

pub struct Port {
    port_num: u16,
}

impl Port {
    pub fn write(&mut self, data: u8) {
            port_byte_out(self.port_num, data);
    }
    pub fn read(&self) -> u8 {
        port_byte_in(self.port_num)
    }

    pub const fn new(port: u16) -> Self {
        Port { port_num: port }
    }
}
pub fn port_byte_in(port: u16) -> u8 {
    let out: u8;
    unsafe { asm!("in al, dx", out("al") out, in("dx") port) }
    out
}

pub fn port_byte_out(port: u16, data: u8) {
    unsafe { asm!("out dx, al", in("al")data, in("edx")port) }
}

// fn port_word_in(port: u16) -> u16 {
//     let out: u16;
//     unsafe { asm!("in ax, dx", out("ax")out, in("dx")port) }
//     out
// }
// 
// fn port_word_out(port: u16, data: u16) {
//     unsafe { asm!("out dx, ax", in("ax")data, in("edx")port) }
// }

// https://stanislavs.org/helppc/ports.html

pub fn tst() {
    port_byte_out(0x3d4, 14);
    let mut pos = (port_byte_in(0x3d5) as u16) << 8;
    println!("0x{:x}", port_byte_in(0x3d5));
    println!("0x{:x}", pos);
    port_byte_out(0x3d4, 15);
    pos += port_byte_in(0x3d5) as u16;
    println!("0x{:x}", pos);

    println!("graphics mode: 0x{:x}", port_byte_in(0x3ce));
}

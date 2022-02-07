use core::fmt::{Arguments, Write};

use crate::ports::{port_byte_in, port_byte_out};
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    static ref SERIAL_COM1: Mutex<SerialPort> = {
        let serial = SerialPort::new(0x3f8);
        serial.init().unwrap();
        Mutex::new(serial)
    };
}

struct SerialPort {
    port: u16,
}

impl SerialPort {
    fn new(port: u16) -> Self {
        SerialPort { port }
    }
    fn is_transmit_empty(&self) -> u8 {
        port_byte_in(self.port + 5) & 0x20
    }

    fn write_byte(&self, a: u8) {
        while self.is_transmit_empty() == 0 {}
        port_byte_out(self.port, a);
    }

    pub fn init(&self) -> Result<(), ()> {
        // adapted from https://wiki.osdev.org/Serial_Ports
        port_byte_out(self.port + 1, 0x00); // Disable all interrupts
        port_byte_out(self.port + 3, 0x80); // Enable DLAB (set baud rate divisor)
        port_byte_out(self.port , 0x03); // Set divisor to 3 (lo byte) 38400 baud
        port_byte_out(self.port + 1, 0x00); //                  (hi byte)
        port_byte_out(self.port + 3, 0x03); // 8 bits, no parity, one stop bit
        port_byte_out(self.port + 2, 0xC7); // Enable FIFO, clear them, with 14-byte threshold
        port_byte_out(self.port + 4, 0x0B); // IRQs enabled, RTS/DSR set
        port_byte_out(self.port + 4, 0x1E); // Set in loopback mode, test the serial chip
        port_byte_out(self.port , 0xAE); // Test serial chip (send byte 0xAE and check if serial returns same byte)

        // Check if serial is faulty (i.e: not same byte as sent)
        if port_byte_in(self.port ) != 0xAE {
            Err(())
        } else {
            // If serial is not faulty set it in normal operation mode
            // (not-loopback with IRQs enabled and OUT#1 and OUT#2 bits enabled)
            port_byte_out(self.port + 4, 0x0F);
            Ok(())
        }
    }
}

impl Write for SerialPort {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}

#[doc(hidden)]
pub fn _serial_print(args: Arguments) {
    x86_64::instructions::interrupts::without_interrupts(|| {
        SERIAL_COM1.lock().write_fmt(args).unwrap();
    });
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => ($crate::serial::_serial_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($($arg:tt)*) => ($crate::serial_print!("{}\n", format_args!($($arg)*)));
}

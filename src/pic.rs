//! Intel 8259 Programmable Interrupt Controller
//! https://docs.rs/crate/pic8259/0.10.1/source/src/lib.rs
//!

use crate::ports::Port;
const CMD_INIT: u8 = 0x11;
const CMD_END_OF_INTERRUPT: u8 = 0x20;

struct Pic {
    offset: u8,
    command_port: crate::ports::Port,
    data_port: crate::ports::Port,
}

impl Pic {
    fn handles_interrupt(&self, interrupt_id: u8) -> bool {
        self.offset <= interrupt_id && interrupt_id < self.offset + 8
    }

    /// # Safety
    /// Incorrect offsets cause UB
    unsafe fn end_of_interrupt(&mut self) {
        self.command_port.write(CMD_END_OF_INTERRUPT);
    }

    /// # Safety
    /// Incorrect offsets cause UB
    unsafe fn read_mask(&mut self) -> u8 {
        self.data_port.read()
    }

    /// # Safety
    /// Incorrect offsets cause UB
    unsafe fn write_mask(&mut self, mask: u8) {
        self.data_port.write(mask);
    }
}

/// A pair of chained PIC controllers.  This is the standard setup on x86.
pub struct ChainedPics {
    pics: [Pic; 2],
}

impl ChainedPics {
    /// # Safety
    /// Incorrect offsets cause UB
    pub const unsafe fn new(offset1: u8, offset2: u8) -> Self {
        ChainedPics {
            pics: [
                Pic {
                    offset: offset1,
                    command_port: Port::new(0x20),
                    data_port: Port::new(0x21),
                },
                Pic {
                    offset: offset2,
                    command_port: Port::new(0xA0),
                    data_port: Port::new(0xA1),
                },
            ],
        }
    }

    /// # Safety
    /// Incorrect offsets cause UB
    pub unsafe fn initialize(&mut self) {
        let mut wait_port: Port = Port::new(0x80);
        let mut wait = || wait_port.write(0);
        let saved_masks = self.read_masks();

        self.pics[0].command_port.write(CMD_INIT);
        wait();
        self.pics[1].command_port.write(CMD_INIT);
        wait();

        self.pics[0].data_port.write(self.pics[0].offset);
        wait();
        self.pics[1].data_port.write(self.pics[1].offset);
        wait();

        self.pics[0].data_port.write(4);
        wait();
        self.pics[1].data_port.write(2);
        wait();

        const MODE_8086: u8 = 0x01;
        self.pics[0].data_port.write(MODE_8086);
        wait();
        self.pics[1].data_port.write(MODE_8086);
        wait();

        self.write_masks(saved_masks[0], saved_masks[1]);
    }

    /// # Safety
    /// Incorrect offsets cause UB
    pub unsafe fn read_masks(&mut self) -> [u8; 2] {
        [self.pics[0].read_mask(), self.pics[1].read_mask()]
    }

    /// # Safety
    /// Incorrect offsets cause UB
    pub unsafe fn write_masks(&mut self, mask1: u8, mask2: u8) {
        self.pics[0].write_mask(mask1);
        self.pics[1].write_mask(mask2);
    }

    /// disable all pics
    /// # Safety
    /// Incorrect offsets cause UB
    pub unsafe fn disable(&mut self) {
        self.write_masks(u8::MAX, u8::MAX)
    }

    /// # Safety
    /// Incorrect offsets cause UB
    pub fn handles_interrupt(&self, interrupt_id: u8) -> bool {
        self.pics.iter().any(|p| p.handles_interrupt(interrupt_id))
    }

    /// # Safety
    /// Incorrect offsets cause UB
    pub unsafe fn notify_end_of_interrupt(&mut self, interrupt_id: u8) {
        if self.handles_interrupt(interrupt_id) {
            if self.pics[1].handles_interrupt(interrupt_id) {
                self.pics[1].end_of_interrupt();
            }
            self.pics[0].end_of_interrupt();
        }
    }
}

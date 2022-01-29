#![no_std]
#![no_main]

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop { /* unrecoverable panic attack */ }
}

static BUONGIORNO: &[u8] = b"Buongiorno";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buf = 0xb8000 as *mut u8; // address
    for (i, &byte) in BUONGIORNO.iter().enumerate() {
        unsafe {
            // C says hi
            *vga_buf.offset(i as isize * 2) = byte;
            *vga_buf.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop { /* nothing */ }
}

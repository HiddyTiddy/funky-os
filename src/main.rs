#![no_std]
#![no_main]

mod vga_buffer;
mod ports;
mod video;

use core::panic::PanicInfo;

use crate::{ports::tst, video::video_tmp};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    eprintln!("Kernel {}", info);
    loop { /* unrecoverable panic attack */ }
}



#[no_mangle]
pub extern "C" fn _start() -> ! {

    println!("hi");
    println!("!!!");
    tst();

    // video_tmp()

    loop { /* nothing */ }
}

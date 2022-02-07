#![no_std]
#![no_main]
// nightly stuff
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

use core::panic::PanicInfo;
use os::{eprintln, hlt_loop, println, serial_println};

// use crate::ports::tst;
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    eprintln!("Kernel {}", info);
    hlt_loop();

    /* unrecoverable panic attack */
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // init
    os::init();

    // println!("hi");
    // println!("!!!");
    // //tst();
    // println!("{}", str_to_int(&[b'1', b'0', b'5']));
    // println!("{}", str_to_int(&[b'0', b'0', b'5']));

    println!("evolve to crab.");
    serial_println!("hello to host");
    // port_byte_out(port+3, 0x80);

    // port_byte_out(port  , 0x03);
    // port_byte_out(port+1, 0x00);
    // port_byte_out(port+3, 0x03);
    // port_byte_out(port+2, 0xc7);
    // port_byte_out(port+4, 0x0b);
    // port_byte_out(port+1, 0x01);

    // port_byte_out(port, b'h');

    os::color_test();
    // println!("still alive");

    // video_tmp()

    hlt_loop();
}

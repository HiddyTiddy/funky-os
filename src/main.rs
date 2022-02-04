#![no_std]
#![no_main]
// nightly stuff
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner)]

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("running {} tests", tests.len());
    for test in tests {
        test();
    }
}

use core::panic::PanicInfo;
use os::utils::str_to_int;
use os::{eprintln, println, interrupts};

// use crate::ports::tst;
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    eprintln!("Kernel {}", info);
    loop { /* unrecoverable panic attack */ }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // init
    interrupts::init_idt();
    x86_64::instructions::interrupts::int3();

    println!("hi");
    println!("!!!");
    //tst();
    println!("{}", str_to_int(&[b'1', b'0', b'5']));
    println!("{}", str_to_int(&[b'0', b'0', b'5']));

    fn lol_die() {
        lol_die();
    }
    lol_die();

    os::color_test();
    println!("still alive");

    // video_tmp()

    loop { /* nothing */ }
}

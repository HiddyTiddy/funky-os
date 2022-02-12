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

extern crate alloc;

use alloc::boxed::Box;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use os::{eprintln, hlt_loop, memory::{self, BootInfoFrameAllocator}, println, serial_println, allocator};
use x86_64::VirtAddr;

// use crate::ports::tst;
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    eprintln!("Kernel {}", info);
    hlt_loop();

    /* unrecoverable panic attack */
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
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

    //os::color_test();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::new(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed lol");



    println!("ait");
    // video_tmp()
    let x = Box::new(12);

    hlt_loop();
}

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

mod vga_buffer;
// mod ports;
// mod video;
mod utils;
mod interrupts;

use core::panic::PanicInfo;

use crate::utils::str_to_int;

// use crate::ports::tst;
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    eprintln!("Kernel {}", info);
    loop { /* unrecoverable panic attack */ }
}

fn color_test() {
    println!("       40m     41m     42m     43m     44m     45m     46m     47m     ");
    println!("30m  \x1b[30m \x1b[30m\x1b[40m ... \x1b[0m   \x1b[30m\x1b[41m ... \x1b[0m   \x1b[30m\x1b[42m ... \x1b[0m   \x1b[30m\x1b[43m ... \x1b[0m   \x1b[30m\x1b[44m ... \x1b[0m   \x1b[30m\x1b[45m ... \x1b[0m   \x1b[30m\x1b[46m ... \x1b[0m   \x1b[30m\x1b[47m ... \x1b[0m   ");
println!("90m  \x1b[90m \x1b[90m\x1b[40m ... \x1b[0m   \x1b[90m\x1b[41m ... \x1b[0m   \x1b[90m\x1b[42m ... \x1b[0m   \x1b[90m\x1b[43m ... \x1b[0m   \x1b[90m\x1b[44m ... \x1b[0m   \x1b[90m\x1b[45m ... \x1b[0m   \x1b[90m\x1b[46m ... \x1b[0m   \x1b[90m\x1b[47m ... \x1b[0m   ");
println!("31m  \x1b[31m \x1b[31m\x1b[40m ... \x1b[0m   \x1b[31m\x1b[41m ... \x1b[0m   \x1b[31m\x1b[42m ... \x1b[0m   \x1b[31m\x1b[43m ... \x1b[0m   \x1b[31m\x1b[44m ... \x1b[0m   \x1b[31m\x1b[45m ... \x1b[0m   \x1b[31m\x1b[46m ... \x1b[0m   \x1b[31m\x1b[47m ... \x1b[0m   ");
println!("91m  \x1b[91m \x1b[91m\x1b[40m ... \x1b[0m   \x1b[91m\x1b[41m ... \x1b[0m   \x1b[91m\x1b[42m ... \x1b[0m   \x1b[91m\x1b[43m ... \x1b[0m   \x1b[91m\x1b[44m ... \x1b[0m   \x1b[91m\x1b[45m ... \x1b[0m   \x1b[91m\x1b[46m ... \x1b[0m   \x1b[91m\x1b[47m ... \x1b[0m   ");
println!("32m  \x1b[32m \x1b[32m\x1b[40m ... \x1b[0m   \x1b[32m\x1b[41m ... \x1b[0m   \x1b[32m\x1b[42m ... \x1b[0m   \x1b[32m\x1b[43m ... \x1b[0m   \x1b[32m\x1b[44m ... \x1b[0m   \x1b[32m\x1b[45m ... \x1b[0m   \x1b[32m\x1b[46m ... \x1b[0m   \x1b[32m\x1b[47m ... \x1b[0m   ");
println!("92m  \x1b[92m \x1b[92m\x1b[40m ... \x1b[0m   \x1b[92m\x1b[41m ... \x1b[0m   \x1b[92m\x1b[42m ... \x1b[0m   \x1b[92m\x1b[43m ... \x1b[0m   \x1b[92m\x1b[44m ... \x1b[0m   \x1b[92m\x1b[45m ... \x1b[0m   \x1b[92m\x1b[46m ... \x1b[0m   \x1b[92m\x1b[47m ... \x1b[0m   ");
println!("33m  \x1b[33m \x1b[33m\x1b[40m ... \x1b[0m   \x1b[33m\x1b[41m ... \x1b[0m   \x1b[33m\x1b[42m ... \x1b[0m   \x1b[33m\x1b[43m ... \x1b[0m   \x1b[33m\x1b[44m ... \x1b[0m   \x1b[33m\x1b[45m ... \x1b[0m   \x1b[33m\x1b[46m ... \x1b[0m   \x1b[33m\x1b[47m ... \x1b[0m   ");
println!("93m  \x1b[93m \x1b[93m\x1b[40m ... \x1b[0m   \x1b[93m\x1b[41m ... \x1b[0m   \x1b[93m\x1b[42m ... \x1b[0m   \x1b[93m\x1b[43m ... \x1b[0m   \x1b[93m\x1b[44m ... \x1b[0m   \x1b[93m\x1b[45m ... \x1b[0m   \x1b[93m\x1b[46m ... \x1b[0m   \x1b[93m\x1b[47m ... \x1b[0m   ");
println!("34m  \x1b[34m \x1b[34m\x1b[40m ... \x1b[0m   \x1b[34m\x1b[41m ... \x1b[0m   \x1b[34m\x1b[42m ... \x1b[0m   \x1b[34m\x1b[43m ... \x1b[0m   \x1b[34m\x1b[44m ... \x1b[0m   \x1b[34m\x1b[45m ... \x1b[0m   \x1b[34m\x1b[46m ... \x1b[0m   \x1b[34m\x1b[47m ... \x1b[0m   ");
println!("94m  \x1b[94m \x1b[94m\x1b[40m ... \x1b[0m   \x1b[94m\x1b[41m ... \x1b[0m   \x1b[94m\x1b[42m ... \x1b[0m   \x1b[94m\x1b[43m ... \x1b[0m   \x1b[94m\x1b[44m ... \x1b[0m   \x1b[94m\x1b[45m ... \x1b[0m   \x1b[94m\x1b[46m ... \x1b[0m   \x1b[94m\x1b[47m ... \x1b[0m   ");
println!("35m  \x1b[35m \x1b[35m\x1b[40m ... \x1b[0m   \x1b[35m\x1b[41m ... \x1b[0m   \x1b[35m\x1b[42m ... \x1b[0m   \x1b[35m\x1b[43m ... \x1b[0m   \x1b[35m\x1b[44m ... \x1b[0m   \x1b[35m\x1b[45m ... \x1b[0m   \x1b[35m\x1b[46m ... \x1b[0m   \x1b[35m\x1b[47m ... \x1b[0m   ");
println!("95m  \x1b[95m \x1b[95m\x1b[40m ... \x1b[0m   \x1b[95m\x1b[41m ... \x1b[0m   \x1b[95m\x1b[42m ... \x1b[0m   \x1b[95m\x1b[43m ... \x1b[0m   \x1b[95m\x1b[44m ... \x1b[0m   \x1b[95m\x1b[45m ... \x1b[0m   \x1b[95m\x1b[46m ... \x1b[0m   \x1b[95m\x1b[47m ... \x1b[0m   ");
println!("36m  \x1b[36m \x1b[36m\x1b[40m ... \x1b[0m   \x1b[36m\x1b[41m ... \x1b[0m   \x1b[36m\x1b[42m ... \x1b[0m   \x1b[36m\x1b[43m ... \x1b[0m   \x1b[36m\x1b[44m ... \x1b[0m   \x1b[36m\x1b[45m ... \x1b[0m   \x1b[36m\x1b[46m ... \x1b[0m   \x1b[36m\x1b[47m ... \x1b[0m   ");
println!("96m  \x1b[96m \x1b[96m\x1b[40m ... \x1b[0m   \x1b[96m\x1b[41m ... \x1b[0m   \x1b[96m\x1b[42m ... \x1b[0m   \x1b[96m\x1b[43m ... \x1b[0m   \x1b[96m\x1b[44m ... \x1b[0m   \x1b[96m\x1b[45m ... \x1b[0m   \x1b[96m\x1b[46m ... \x1b[0m   \x1b[96m\x1b[47m ... \x1b[0m   ");
println!("37m  \x1b[37m \x1b[37m\x1b[40m ... \x1b[0m   \x1b[37m\x1b[41m ... \x1b[0m   \x1b[37m\x1b[42m ... \x1b[0m   \x1b[37m\x1b[43m ... \x1b[0m   \x1b[37m\x1b[44m ... \x1b[0m   \x1b[37m\x1b[45m ... \x1b[0m   \x1b[37m\x1b[46m ... \x1b[0m   \x1b[37m\x1b[47m ... \x1b[0m   ");
println!("97m  \x1b[97m \x1b[97m\x1b[40m ... \x1b[0m   \x1b[97m\x1b[41m ... \x1b[0m   \x1b[97m\x1b[42m ... \x1b[0m   \x1b[97m\x1b[43m ... \x1b[0m   \x1b[97m\x1b[44m ... \x1b[0m   \x1b[97m\x1b[45m ... \x1b[0m   \x1b[97m\x1b[46m ... \x1b[0m   \x1b[97m\x1b[47m ... \x1b[0m   ");
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

    color_test();
    unsafe {
        *(0xcafebabe as *mut u64) = 42
    };
    println!("still alive");

    // video_tmp()


    loop { /* nothing */ }
}

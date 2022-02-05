#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod vga_buffer;
pub mod ports;
pub mod serial;
// pub mod video;
pub mod gdt;
pub mod interrupts;
pub mod utils;
pub fn color_test() {
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

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T where T: Fn() {
    fn run(&self) {
        // todo serial println
        println!("{}...\t", core::any::type_name::<T>());
        self();
        println!("ok.");
    }
}

//#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    println!("running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn test_panic_handler(info: &core::panic::PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

/// Entry point for `cargo xtest`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // init();
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    test_panic_handler(info)
}


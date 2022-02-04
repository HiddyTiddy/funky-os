#![no_std]
#![no_main]

#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

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

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {

}

pub mod vga_buffer;
// pub mod ports;
// pub mod video;
pub mod utils;
pub mod interrupts;
pub mod gdt;

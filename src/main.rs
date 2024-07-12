// since std lib requires os level features, we dont use it
#![no_std]
// we are not using main the standard entrypoint and providing 
// our own, because the standrd one is os dependent
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]

// PRINTING MODULE
use core::panic::PanicInfo;
use rust_os::println;

// standard panic handler
#[cfg(not(test))]
// current panic handler - can't do much so just loop
// because we loop, so never return, therefore type !
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // now with println! macro impl, we can output the 
    // panic info to vga buffer
    println!("{}", _info);
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

// no mangle tells the compiler to NOT change the name of the fn
#[no_mangle]
pub extern "C" fn _start() -> !{
    // this is the entrypoint, and it is named _start 
    // because the linker looks for _start as the entrypoint

    // // VERSION1: print to vga buffer manually via 
    // // unsafe raw pointer arithmetics with no bound checking
    // // why do we ned to cast it as a raw pointer?
    // let vga_buffer = 0xb8000 as *mut u8;

    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }

    // // VERSION2: print using a public fn from vga_buffer
    // vga_buffer::print_something();


    // //VERSION3: print using global Writer object
    // use core::fmt::Write;
    // // two different methods of writing to the vga buffer
    // vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    // write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 4.20, 69).unwrap();


    // VERSION4: print using println! macro now implemented
    println!("Hello World{}", "!");
    println!("We are now printing from the println! macro through a 
        well defined, safe VGA buffer interface :)");

    // inits the interrupt module (so far), maybe come up with better name than init?
    rust_os::init();

    // Uncomment below to trigger double fault interrupt
    // will trigger page fault -> page fault not impl so double fault is triggered
    // DEADBEEF PEPELAUGH
    // unsafe { 
    //     *(0xdeadbeef as *mut u8) = 42;
    // };

    // // why does this line cause a breakpoint?
    // // int3 is the code for a breakpoint interrupt 
    // // we import x86_64 crate from toml
    // // uncomment for a breakpoint exception
    // x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("It did not crash pepelaugh");

    panic!("Testing panic handler");
    loop {}
}

// now in lib.rs
//
//
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
//
// #[repr(u32)]
// pub enum QemuExitCode {
//     Success = 0x10,
//     Failed = 0x11,
// }
// 
// pub fn exit_qemu(exit_code: QemuExitCode) {
//     use x86_64::instructions::port::Port;
// 
//     unsafe {
//         // why does port have to be mutable here?
//         // ans: because we are writing to hw
//         let mut port = Port::new(0xf4);
//         port.write(exit_code as u32);
//     }
// }


// // TESTING MODULE
// Mod serial;
// 
// Pub trait Testable {
//     fn run(&self) -> ();
// }
// 
// // blanket impl for any type T which impl Fn() trait
// Impl<T> Testable for T 
// Where 
// T: Fn(),
// {
//     fn run(&self) {
//         serial_print!("{}...\t", core::any::type_name::<T>());
//         self();
//         serial_println!("[ok]");
//     }
// }
// 
// #[cfg(test)]
// Pub fn test_runner(tests: &[&dyn Testable]) {
//     serial_println!("Running {} tests", tests.len());
//     for test in tests {
//         test.run();
//     }
// 
//     loop {}
//     // now with the exit fn
//     exit_qemu(QemuExitCode::Success);
// }
// 
// RUN THIS PANIC HANDLER WHEN WE ARE TESTING
#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // VERSION1
    // serial_println!("[failed]\n");
    // serial_println!("Error: {}\n", _info);
    // exit_qemu(QemuExitCode::Failed);
    // loop {}
    // // now with lib.rs
    rust_os::test_panic_handler(_info)
}

// trivial test
#[test_case]
fn trivial_assertion() {
    assert_eq!(0, 0);
}

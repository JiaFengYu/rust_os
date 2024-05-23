// since std lib requires os level features, we dont use it
#![no_std]
// we are not using main the standard entrypoint and providing 
// our own, because the standrd one is os dependent
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}


#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}



mod vga_buffer;

use core::panic::PanicInfo;

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

    #[cfg(test)]
    test_main();

    panic!("Testing panic handler");
    loop {}
}


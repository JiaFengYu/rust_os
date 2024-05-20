// since std lib requires os level features, we dont use it
#![no_std]
// we are not using main the standard entrypoint and providing 
// our own, because the standrd one is os dependent
#![no_main]


use core::panic::PanicInfo;

// current panic handler - can't do much so just loop
// because we loop, so never return, therefore type !
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";
// no mangle tells the compiler to NOT change the name of the fn
#[no_mangle]
pub extern "C" fn _start() -> !{
    // this is the entrypoint, and it is named _start 
    // because the linker looks for _start as the entrypoint
    
    // why do we ned to cast it as a raw pointer?
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}

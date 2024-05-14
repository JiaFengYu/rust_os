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

// no mangle tells the compiler to NOT change the name of the fn
#[no_mangle]
pub extern "C" fn _start() -> !{
    loop {}
}

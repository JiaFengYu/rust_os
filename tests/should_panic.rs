#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
// use rust_os::{exit_qemu, QemuExitCode, serial_println, serial_print};
use rust_os::{serial_println, serial_print};
use rust_os::testing_fns::{exit_qemu, QemuExitCode};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop{}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

// not necessary if we don't use a test runner
//#[test_case]
fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}


// Not really necessary since we panic, we can only really run 1 test
// just run from entrypoint
// // these tests SHOULD panic
// pub fn test_runner(tests: &[&dyn Fn()]) {
//     serial_println!("Running {} tests", tests.len());
//     for test in tests {
//         test();
//         // if reached here, we didn't panic 
//         // so we fked up
//         serial_println!("[test did not panic]");
//         exit_qemu(QemuExitCode::Failed);
//     }
//     // reached here means we aint run no test
//     exit_qemu(QemuExitCode::Success);
// }


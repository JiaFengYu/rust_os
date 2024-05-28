#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::testing_fns::test_runner)]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]

pub mod serial;
pub mod vga_buffer;
pub mod testing_fns;

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(lhy_blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use lhy_blog_os::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) ->! {
    println!("{}", _info);
    loop{}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    lhy_blog_os::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    // panic!("Some panic message");
    loop{}
}

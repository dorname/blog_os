
#![no_std] // 不链接 Rust 标准库
#![no_main] // 禁用所有 Rust 层级的入口点
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]


#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os::println;
/// 这个函数将在 panic 时被调用
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("my_panic:{}", _info);
    loop {}
}
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    blog_os::test_panic_handler(info);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {

    println!("Hello World{}", "!");
    
    #[cfg(test)]
    test_main();
    
    loop {}
}

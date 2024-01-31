
#![no_std] // 不链接 Rust 标准库
#![no_main] // 禁用所有 Rust 层级的入口点
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]


#![reexport_test_harness_main = "test_main"]

mod vga_buffer;

use core::panic::PanicInfo;

/// 这个函数将在 panic 时被调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("my_panic:{}", _info);
    loop {}
}

// static HELLO: &[u8] = b"Hello World!";

// #[no_mangle]
// pub extern "C" fn _start() -> ! {
//     let vga_buffer = 0xb8000 as *mut u8;

//     for (i, &byte) in HELLO.iter().enumerate() {
//         unsafe {
//             *vga_buffer.offset(i as isize * 2) = byte;
//             *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
//         }
//     }

//     loop {}
// }
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    // vga_buffer::WRITER.lock().write_str("Hello again again").unwrap();
    // write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();
    println!("Hello World{}", "!");
    // panic!("This is a panic message");
    
    #[cfg(test)]
    test_main();
    
    loop {}
}


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
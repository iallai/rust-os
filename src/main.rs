#![no_std] //不使用标准库
#![no_main] //禁用所有 rust-level的入口

use core::panic::PanicInfo;

static HELLO: &[u8] = b"hello world!";

#[no_mangle] // 不允许改名
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8; //raw pointer
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
            //每个字符由一个 ASCII byte 和 color byte组成
        }
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// rustup target add thumbv7em-none-eabihf
// cargo build --target thumbv7em-none-eabihf

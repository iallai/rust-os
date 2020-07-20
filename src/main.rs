#![no_std] //不使用标准库
#![no_main] //禁用所有 rust-level的入口

use core::panic::PanicInfo;

#[no_mangle] // 不允许改名
pub extern "C" fn _start() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// rustup target add thumbv7em-none-eabihf
// cargo build --target thumbv7em-none-eabihf

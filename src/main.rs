// src/main.rs

#![no_std] // Rust 표준 라이브러리를 링크하지 않도록 합니다
#![no_main] // Rust 언어에서 사용하는 실행 시작 지점 (main 함수)을 사용하지 않습니다

use core::panic::PanicInfo;

/// 패닉이 일어날 경우, 이 함수가 호출됩니다.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
#![no_std]
#![no_main]

use core::panic::PanicInfo;

use vga_buffer::print_something;

static HELLO: &[u8] = b"Hello World!";
mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in HELLO.iter().enumerate(){
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xcf;
        }
    }
    print_something();
    loop{}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
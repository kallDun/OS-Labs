#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod vga_buffer;

use core::fmt::Write;
use core::panic::PanicInfo;
use core::ptr::write;
use crate::vga_buffer::{Alignment, Color, Screen};

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut screen = Screen::new(Color::Yellow, Alignment::Center);
    for i in 0..100 {
        write!(screen, "Number {}\n", i);
    }
    loop {}
}
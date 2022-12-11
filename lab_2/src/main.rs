#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod vga_buffer;
mod game_of_life;

use core::fmt::Write;
use core::panic::PanicInfo;
use core::ptr::write;
use crate::game_of_life::game_of_life;
use crate::vga_buffer::{Alignment, Color, Screen};

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let mut screen = Screen::new(Color::LightGreen, Alignment::Left);
    write!(screen, "{:?}", _info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut screen = Screen::new(Color::LightGreen, Alignment::Left);
    /*for i in 0..100 {
        write!(screen, "Number {}\n", i);
    }
    loop {}*/
    game_of_life(&mut screen);
}
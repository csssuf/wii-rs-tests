#![feature(naked_functions, start)]

#![no_std]
#![no_main]

mod crt;
mod panic;

#[no_mangle]
fn main() {
    loop {}
}

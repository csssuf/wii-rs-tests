#![feature(naked_functions)]

#![no_std]
#![no_main]

mod crt;
mod panic;

#[no_mangle]
fn main() {
    loop {}
}

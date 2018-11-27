#[naked]
#[no_mangle]
pub unsafe extern fn _start() -> ! {
    let mut x = 0;

    loop { x += 1; }
}

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use pic32 as _;

#[no_mangle]
fn main() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

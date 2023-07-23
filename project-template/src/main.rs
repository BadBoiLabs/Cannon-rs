#![no_std]
#![no_main]
#![feature(core_intrinsics)]
#![feature(alloc_error_handler)]

/// Defines the size of the heap in bytes
/// Changing this will change the size of the resulting json file built by converting the elf file
/// How big you can make this depends on the program size but it should be possible to make it very large (close to 4GB).
/// See https://image1.slideserve.com/3443033/memory-map-l.jpg
const HEAP_SIZE: usize = 0x400000;

use cannon_io::prelude::*;
use cannon_heap::init_heap;

extern crate alloc;

/// Main entrypoint for a verifiable computation
#[no_mangle]
pub extern "C" fn _start() {
    init_heap!(HEAP_SIZE);

    print("Lets do something amazing!\n");

    exit(0); // 0 code indicates success
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let msg = alloc::format!("Panic: {}", info);
    let _ = print(&msg);
    exit(2);
}

#[alloc_error_handler]
fn alloc_error_handler(_layout: alloc::alloc::Layout) -> ! {
    let _ = print("alloc error! (probably out of memory)");
    exit(3);
}

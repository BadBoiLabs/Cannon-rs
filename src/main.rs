#![feature(alloc_error_handler)] // no_std and allocator support is not stable.
#![feature(stdsimd)] // for `mips::break_`. If desired, this could be replaced with asm.
#![no_std]
#![no_main]

extern crate alloc;
extern crate rlibc; // memcpy, and friends

mod heap;
mod iommu;

/// Main entrypoint for a verifiable computation
#[no_mangle]
pub extern "C" fn _start() {
    unsafe { heap::init() };
    // grab the input hash
    let input_hash = iommu::input_hash();

    // Do something amazing (‾⌣‾)

    // Write the output
    immou::output([0; 32]);
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // Uncomment code below if you're in trouble
    /*
    let msg = alloc::format!("Panic: {}", info);
    iommu::print(&msg);
    */

    unsafe {
        core::arch::mips::break_();
    }
}

#[alloc_error_handler]
fn alloc_error_handler(_layout: alloc::alloc::Layout) -> ! {
    // NOTE: avoid `panic!` here, technically, it might not be allowed to panic in an OOM situation.
    //       with panic=abort it should work, but it's no biggie use `break` here anyway.
    unsafe {
        core::arch::mips::break_();
    }
}

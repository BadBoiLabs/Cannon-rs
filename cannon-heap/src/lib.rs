#![no_std]
///! This is actually just a wrapper around linked_list_allocator that allows it to work in our environment
///! Different allocator can be used if desired
use core::alloc::{GlobalAlloc, Layout};
use core::cell::RefCell;
use core::mem::MaybeUninit;
use core::ptr::{self, NonNull};
struct Alloc {
    heap: RefCell<linked_list_allocator::Heap>,
}

impl Alloc {
    const fn new() -> Self {
        Self {
            heap: RefCell::new(linked_list_allocator::Heap::empty()),
        }
    }
}

unsafe impl GlobalAlloc for Alloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.heap
            .borrow_mut()
            .allocate_first_fit(layout)
            .ok()
            .map_or(ptr::null_mut(), |allocation| allocation.as_ptr())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.heap
            .borrow_mut()
            .deallocate(NonNull::new_unchecked(ptr), layout)
    }
}

#[global_allocator]
static mut ALLOCATOR: Alloc = Alloc::new();

pub unsafe fn init(heap: &mut [MaybeUninit<u8>]) {
    ALLOCATOR
        .heap
        .borrow_mut()
        .init(heap.as_mut_ptr() as *mut u8, heap.len())
}

#[macro_export]
macro_rules! init_heap {
    ( $x:expr ) => {{
        use cannon_heap::init;
        use core::mem::MaybeUninit;
        static mut HEAP: [MaybeUninit<u8>; $x] = [MaybeUninit::uninit(); $x];
        unsafe { init(&mut HEAP) }
    }};
}

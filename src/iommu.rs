//! This module is used when the state machine compiled into MIPS to interact with the host
//! environment. The host environment is either the prover or the onchain one step verifier.

use alloc::{collections::btree_map::BTreeMap, vec::Vec};
use core::ptr;

type H256 = [u8; 32];

/// The address of the input hash.
const PTR_INPUT_HASH: usize = 0x30000000;
/// The address where the output hash is written at the end of execution.
const PTR_OUTPUT_HASH: usize = 0x30000804;
/// The address where a special magic value is written at the end of execution.
const PTR_MAGIC: usize = 0x30000800;
/// The address where the preimage hash for the preimage oracle is written by the guest.
const PTR_PREIMAGE_ORACLE_HASH: usize = 0x30001000;
/// The address where the preimage oracle output size is written by the host.
const PTR_PREIMAGE_ORACLE_SIZE: usize = 0x31000000;
/// The address where the preimage oracle output data is written by the host.
const PTR_PREIMAGE_ORACLE_DATA: usize = 0x31000004;

/// Loads the input hash from the host environment.
pub fn input_hash() -> H256 {
    unsafe { ptr::read_volatile(PTR_INPUT_HASH as *const H256) }
}

/// Prepares the guest envrionment to exiting. Writes the output hash and the magic to be read by
/// the host and then halts the execution.
pub fn output(hash: H256) -> ! {
    unsafe {
        ptr::write_volatile(PTR_MAGIC as *mut u32, 0x1337f00d);
        ptr::write_volatile(PTR_OUTPUT_HASH as *mut H256, hash);
        ffi::halt();
    }
}

/// Request a preimage from the oracle.
///
/// The returned slice is valid until the end of the program.
pub fn preimage(hash: H256) -> Option<&'static [u8]> {
    // The cache of all requested preimages to avoid going via the host boundary every time.
    //
    // Under MIPS this is running exclusively in single-threaded mode. We could've avoided using
    // a Mutex, but it seems to be fine. Uncontended use is just atomic writes.
    static mut PREIMAGE_CACHE: Option<BTreeMap<H256, Vec<u8>>> = None;

    // assume the given reference is valid for the whole program lifetime.
    let eternalize = |v: &Vec<u8>| -> &'static [u8] {
        // SAFETY: this is safe because we are creating the slice from the pointer and the size
        //         that were already produced by a vec.
        //
        //         use-after-free is also a non concern because the vec is owned by the cache and
        //         the cache is never pruned.
        unsafe { core::slice::from_raw_parts(v.as_ptr(), v.len()) }
    };

    // Check if the preimage is already cached.
    unsafe {
        let mut preimage_cache = match PREIMAGE_CACHE {
            Some(ref mut cache) => cache,
            None => {
                let cache = BTreeMap::new();
                PREIMAGE_CACHE = Some(cache);
                PREIMAGE_CACHE.as_mut().unwrap()
            }
        };

        if let Some(preimage) = preimage_cache.get(&hash) {
            return Some(eternalize(preimage));
        }

        *(PTR_PREIMAGE_ORACLE_HASH as *mut [u8; 32]) = hash;

        ffi::preimage_oracle();

        // Read the size of the preimage. It seems to be BE, so no conversion needed.
        let size = *(PTR_PREIMAGE_ORACLE_SIZE as *const u32);
        if size == 0 {
            return None;
        }

        // Read the preimage.
        //
        // SAFETY: The pointer is aligned by definition and is not null.
        let preimage =
            core::slice::from_raw_parts(PTR_PREIMAGE_ORACLE_DATA as *const u8, size as usize)
                .to_vec();

        // if arbitrary_state_machine::keccak256(&preimage) != hash {
        //     panic!("preimage oracle returned invalid preimage");
        // }

        let s = eternalize(&preimage);
        preimage_cache.insert(hash, preimage);
        Some(s)
    }
}

pub fn print(s: &str) {
    unsafe {
        ffi::write(1, s.as_ptr(), s.len());
    }
}

mod ffi {
    //! See asm.S
    extern "C" {
        pub fn halt() -> !;
        pub fn preimage_oracle();
        pub fn write(fd: usize, buf: *const u8, count: usize);
    }
}

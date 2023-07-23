//! A crate to allow Cannon guest programs written in Rust to communicate with the host
//!
//! The Cannon host provides a number of ways for the guest to write outputs and request data. This crate provides simple and safe wrappers
//! around the low level syscalls that implement these features. Note that this crate can only be used when building for a MIPS32 target.
//!
//! The main features of this crate are exposed in the prelude which can be imported with `import cannon_io::prelude::*;`.
//! This imports the `oracle_reader`, `exit`, and `print` functions along with the `PreimageKey` and `Read` traits.

#![no_std]
#![feature(asm_experimental_arch)]

extern crate alloc;

pub mod logger;
pub mod oracle;
pub mod syscalls;

/// Prelude imports commonly used functions and traits
pub mod prelude {
    pub use crate::oracle::{oracle_reader, PreimageKey, Read};
    pub use crate::syscalls::{exit, print, read_hint, write_hint};
}

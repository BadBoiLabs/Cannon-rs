//! Low level access to syscalls that are understood by the minimal Cannon kernel
//! Using these can be dangerous. Prefer to use the oracle_reader if possible

use raw::{syscall1, syscall3};

#[cfg(target_arch = "mips")]
mod raw;
// Provide a mock non-mips implementation so that rust-analyzer and cargo check work correctly
#[cfg(not(target_arch = "mips"))]
mod raw {
    pub unsafe fn syscall1(_: u32, _: u32) -> u32 {
        todo!("Crate can only work on MIPS target")
    }
    pub unsafe fn syscall3(_: u32, _: u32, _: u32, _: u32) -> Result<u32, i32> {
        todo!("Crate can only work on MIPS target")
    }
}

enum FileDescriptor {
    StdOut = 1,
    HintRead = 3,
    HintWrite = 4,
    PreimageRead = 5,
    PreimageWrite = 6,
}
enum SyscallNo {
    Exit = 4246,
    Read = 4003,
    Write = 4004,
}

#[derive(Debug)]
pub enum SyscallError {
    Code(u32),
}

impl From<i32> for SyscallError {
    fn from(code: i32) -> Self {
        SyscallError::Code(code as u32)
    }
}

type Result<T> = core::result::Result<T, SyscallError>;

pub fn print(s: &str) -> Result<u32> {
    write(FileDescriptor::StdOut, s.as_bytes())
}

pub fn write_preimage(key: &[u8]) -> Result<u32> {
    write(FileDescriptor::PreimageWrite, key)
}

pub fn read_preimage(out: &mut [u8]) -> Result<u32> {
    read(FileDescriptor::PreimageRead, out)
}

pub fn write_hint(key: [u8; 32]) -> Result<u32> {
    write(FileDescriptor::HintWrite, &key)
}

pub fn read_hint(out: &mut [u8]) -> Result<u32> {
    read(FileDescriptor::HintRead, out)
}

pub fn exit(code: u8) -> ! {
    unsafe {
        syscall1(SyscallNo::Exit as u32, code.into());
        panic!() // just to get the correct never return type
    }
}

fn write(fd: FileDescriptor, buf: &[u8]) -> Result<u32> {
    let result = unsafe {
        syscall3(
            SyscallNo::Write as u32,
            fd as u32,
            buf.as_ptr() as u32,
            buf.len() as u32,
        )
    };
    result.map_err(SyscallError::from)
}

fn read(fd: FileDescriptor, buf: &mut [u8]) -> Result<u32> {
    let result = unsafe {
        syscall3(
            SyscallNo::Read as u32,
            fd as u32,
            buf.as_ptr() as u32,
            buf.len() as u32,
        )
    };
    result.map_err(SyscallError::from)
}

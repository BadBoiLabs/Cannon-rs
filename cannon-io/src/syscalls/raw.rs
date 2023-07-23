// Taken from the syscalls crate https://github.com/jasonwhite/syscalls

// MIPS has the following registers:
//
// | Symbolic Name | Number          | Usage                          |
// | ============= | =============== | ============================== |
// | zero          | 0               | Constant 0.                    |
// | at            | 1               | Reserved for the assembler.    |
// | v0 - v1       | 2 - 3           | Result Registers.              |
// | a0 - a3       | 4 - 7           | Argument Registers 1 ·· · 4.   |
// | t0 - t9       | 8 - 15, 24 - 25 | Temporary Registers 0 · · · 9. |
// | s0 - s7       | 16 - 23         | Saved Registers 0 ·· · 7.      |
// | k0 - k1       | 26 - 27         | Kernel Registers 0 ·· · 1.     |
// | gp            | 28              | Global Data Pointer.           |
// | sp            | 29              | Stack Pointer.                 |
// | fp            | 30              | Frame Pointer.                 |
// | ra            | 31              | Return Address.                |
//
// The following registers are used for args 1-6:
//
// arg1: %a0 ($4)
// arg2: %a1 ($5)
// arg3: %a2 ($6)
// arg4: %a3 ($7)
// arg5: (Passed via user stack)
// arg6: (Passed via user stack)
// arg7: (Passed via user stack)
//
// %v0 is the syscall number.
// %v0 is the return value.
// %v1 is the error code
// %a3 is a boolean indicating that an error occurred.
//
//
// All temporary registers are clobbered (8-15, 24-25).

use core::arch::asm;

/// Issues a raw system call with 1 argument. (e.g. exit)
#[inline]
pub unsafe fn syscall1(n: u32, arg1: u32) -> u32 {
    let mut err: u32;
    let mut ret: u32;
    asm!(
        "syscall",
        inlateout("$2") n => ret,
        lateout("$7") err,
        in("$4") arg1,
        // All temporary registers are always clobbered
        lateout("$8") _,
        lateout("$9") _,
        lateout("$10") _,
        lateout("$11") _,
        lateout("$12") _,
        lateout("$13") _,
        lateout("$14") _,
        lateout("$15") _,
        lateout("$24") _,
        lateout("$25") _,
        options(nostack, preserves_flags)
    );
    if err == 0 {
        ret
    } else {
        ret.wrapping_neg()
    }
}

/// Issues a raw system call with 3 arguments. (e.g. read, write)
#[inline]
unsafe fn syscall3raw(n: u32, arg1: u32, arg2: u32, arg3: u32) -> u32 {
    let mut err: u32;
    let mut ret: u32;
    asm!(
        "syscall",
        inlateout("$2") n => ret,
        lateout("$7") err,
        in("$4") arg1,
        in("$5") arg2,
        in("$6") arg3,
        // All temporary registers are always clobbered
        lateout("$8") _,
        lateout("$9") _,
        lateout("$10") _,
        lateout("$11") _,
        lateout("$12") _,
        lateout("$13") _,
        lateout("$14") _,
        lateout("$15") _,
        lateout("$24") _,
        lateout("$25") _,
        options(nostack, preserves_flags)
    );
    if err == 0 {
        ret
    } else {
        ret.wrapping_neg()
    }
}

/// Same as above but handles the error code and wraps it in a Result.
#[inline]
pub unsafe fn syscall3(nr: u32, a1: u32, a2: u32, a3: u32) -> Result<u32, i32> {
    let value = syscall3raw(nr, a1, a2, a3);
    if value > -4096isize as u32 {
        // Truncation of the error value is guaranteed to never occur due to
        // the above check. This is the same check that musl uses:
        // https://git.musl-libc.org/cgit/musl/tree/src/internal/syscall_ret.c?h=v1.1.15
        Err(-(value as i32))
    } else {
        Ok(value)
    }
}

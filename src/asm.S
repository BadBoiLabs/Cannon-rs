.global halt
.global preimage_oracle
.global write

halt:
    # the actual halt condition is pc=0x5ead0000, and that's what the 4246 (exit) syscall will do.
    # Just rely on that to avoid accidental compatibility hazards.
    li $v0, 4246
    syscall

brk:
    # this syscall does not do anything, it just returns the current program break address.
    li $v0, 4045
    syscall
    jr $ra
    nop

preimage_oracle:
    # 4020 is the syscall number for getpid. In cannon, it is used for triggering the preimage 
    # oracle.
    li $v0, 4020
    syscall
    jr $ra
    nop

write:
    li $v0, 4004
    syscall
    jr $ra
    nop

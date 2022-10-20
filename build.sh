#!/usr/bin/env bash
set -e

ELF_NAME={{project-name}}

mkdir -p build

RUSTFLAGS="-Clink-arg=-e_start" cargo build --release --target=mips-unknown-none.json -Zbuild-std=core,std,alloc,panic_abort,compiler_builtins -Zbuild-std-features=compiler-builtins-mem

python3 -m venv venv

source venv/bin/activate
pip3 install -r requirements.txt
./elf2bin.py ./target/mips-unknown-none/release/$ELF_NAME ./build/out.bin
deactivate

# Cannon Test

Bootstraped with [Rust-Cannon-Template](https://github.com/badboilabs/rust-cannon-template)

## Prerequisites

- This repo uses `just` for build scripts. Install it with

```shell
cargo install just
```

- Cannon emulator is required for patching and running programs. See https://github.com/ethereum-optimism/optimism/tree/develop/cannon

- Docker installation is recommended for building

## Building with Docker

Preferred build method is using docker. Run with

```shell
just build
```


## Building Locally (Tested on Ubuntu 22.02 only)
### Dependencies

```shell
sudo apt install \
    build-essential \
    g++-mips-linux-gnu \
    libc6-dev-mips-cross \
    llvm \
    clang \
    python3 python3.8-venv python3-pip 
```
### Building

Build an elf with

```shell
cargo build -p cannon-test --release -Zbuild-std
```

patch the elf for Cannon

```shell
cannon load-elf --path ../target/mips-unknown-none/release/cannon-test --patch stack
```

This should produce a `state.json` and `meta.json` which can be used to run the program in the `cannon` emulator.

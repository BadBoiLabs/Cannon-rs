# Rust Cannon Template 🦀💣💥

This repo contains a build system and a minimal Rust program for building MIPS binaries that are executable in the context of [Optimism Cannon](https://github.com/ethereum-optimism/optimism/tree/develop/cannon). 

It is intended to be used as a template with `cargo generate` e.g.

```shell
cargo generate BadBoiLabs/rust-cannon-template
```

## Building

### Building with Docker

Preferred build method is using docker. BadBoiLabs provides a builder image for Cannon-rs projects

```shell
docker run --rm -v `pwd`/..:/code -w="/code" ghcr.io/badboilabs/cannon-rs/builder:main cargo build --release -Zbuild-std 
```

or use the just script provided

```shell
just build
```

## Patching and running in Cannon

Install the `cannon` tool from Optimism (requires Golang installed)

```shell
git clone https://github.com/ethereum-optimism/optimism
cd optimism/cannon
go install .
```

Patch the elf

```shell
cannon load-elf --path ../target/mips-unknown-none/release/{{project-name}} --patch stack
```

Run it in the Cannon emulator

```shell
cannon run --input ./state.json --info-at %100 --stop-at never
```

## Credits

The origins of this amazing work is from @pepyakin in their [Cannon fork](https://github.com/pepyakin/rusty-cannon/). This has evolved into a streamlined Cannon build system and been migrated to the latest version of Cannon

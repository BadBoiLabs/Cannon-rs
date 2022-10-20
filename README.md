# Rust Cannon Template 🦀💣💥

This repo contains a build system and a minimal Rust program for building MIPS binaries that are executable in the context of [Optimism Cannon](https://github.com/ethereum-optimism/cannon). 

It comes with a few barebones helpers for reading the input hashes, writing output and using the pre-image oracle.

It is intended to be used as a template with `cargo generate` e.g.

```shell
cargo generate willemolding/rust-cannon-template
```

## Usage

The template uses Docker for cross-compiling MIPS on any host. First build the docker image by running:

```shell
make docker_image
```

After this a Cannon ready MIPS binary can be build with:
```shell
make build
```

This will write an `out.bin` file to the build directory.

---

Alternatively if you want to experiment in the build environment you can load up an interactive shell with
	
```shell
make docker_image
docker run -it --rm -v $(pwd):/code {{project-name}}/builder bash
```
(replace with your project name as required)

and from there you can run 

```shell
./build.sh
```
to produce the output

## Credits

The majority of this amazing work was done by @pepyakin in their [Cannon fork](https://github.com/pepyakin/rusty-cannon/). This just pulls out the relevant pieces and adds a few quality of life improvements to the build system.

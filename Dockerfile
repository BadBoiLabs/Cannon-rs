FROM ubuntu:20.04

ENV SHELL=/bin/bash
ENV DEBIAN_FRONTEND noninteractive

RUN apt-get update
RUN apt-get install --assume-yes --no-install-recommends \
    build-essential \
    curl \
    g++-mips-linux-gnu \
    libc6-dev-mips-cross \
    make \
    cmake \
    git \
    python3 python3.8-venv python3-pip

RUN pip3 install wheel

ENV CC_mips_unknown_none=mips-linux-gnu-gcc \
    CXX_mips_unknown_none=mips-linux-gnu-g++ \
    CARGO_TARGET_MIPS_UNKNOWN_NONE_LINKER=mips-linux-gnu-gcc

#
# Install Rustup and Rust
#
# Use this specific version of rust. This is needed because versions of rust after this one broke
# support for -Zbuild-std for the embedded targets.
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y --default-toolchain nightly-2022-05-31 --component rust-src
ENV PATH="/root/.cargo/bin:${PATH}"

# Used for build caching
RUN cargo install cargo-chef

WORKDIR /code

COPY . .

RUN git config --global --add safe.directory '*'

# Generate recipe
RUN cargo chef prepare --recipe-path recipe.json

# Download and build depdencies
RUN cargo chef cook --release --recipe-path recipe.json

CMD ["/bin/bash", "build.sh"]

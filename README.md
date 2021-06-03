# Benson Box
[![license: LGPL v3](https://img.shields.io/badge/License-LGPL%20v3-blue.svg)](LICENSE) ![ci status badge](https://github.com/ng8eke/benson/workflows/CI/badge.svg) [![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](docs/CONTRIBUTING.adoc)

Benson Box built on [Substrate](https://github.com/paritytech/substrate).
For getting started and technical guides, please refer to the [Benson Wiki](https://wiki.cennz.net/#/).

## Contributing

All PRs are welcome! Please follow our contributing guidelines [here](docs/CONTRIBUTING.md).

------

## Community

Join our official Benson Discord server 🤗

* Get Benson technical support 🛠
* Meet startups and DApp developers 👯‍♂️
* Learn more about Benson and blockchain 🙌
* Get updates on Benson bounties and grants 💰
* Hear about the latest hackathons, meetups and more 👩‍💻

Join the Discord server by clicking on the badge below!

[![Support Server](https://img.shields.io/discord/801219591636254770.svg?label=Discord&logo=Discord&colorB=7289da&style=for-the-badge)](https://discord.gg/YX2DNVh89)

### Run with Docker

Use the latest Benson docker image to get started quickly
```bash
# Start a local validator on a development chain
$ docker run \
    -p 9933:9933 -p 9944:9944 \
    ng8eke/benson:latest \
    --dev \
    --unsafe-ws-external \
    --unsafe-rpc-external
```

### Run from Source

Follow the steps to build and run a Benson Box from the source code.

#### 1) Set up build environment

For Linux (the example below is for Debian-based machines):
```bash
$ sudo apt install -y build-essential clang cmake gcc git libclang-dev libssl-dev pkg-config
```

For MacOS (via Homebrew):
```bash
$ brew install openssl cmake llvm
```

#### 2) Install Rust

Install Rust on your machine through [here](https://rustup.rs/), and the following rust version and toolchains.
```bash
$ cargo --version
$ rustup install nightly
$ rustup target add --toolchain=nightly wasm32-unknown-unknown
```

#### 3) Build and Run

Clone the repo, build the binary and run it.
```bash
$ git clone https://github.com/ng8eke/benson.git
$ cd benson
$ cargo build --release # or remove  '--release' for quick debug build
$ ./target/release/benson --help

# start a validator node for development
$ ./target/release/benson --dev
```

### Build Docker Image

Prepare your docker engine, and make sure it is running.

```bash
# To use the default image name and tag
$ make 

# To custom your image name and tag
$ IMAGE_NAME='benson' IMAGE_TAG='v1.5.1' DOCKER_BUILD_ARGS='--no-cache --quiet' make build

# Without using make
$ docker build --no-cache -t benson:v1.5.1 .
```

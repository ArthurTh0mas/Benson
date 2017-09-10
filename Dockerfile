FROM  rustlang/rust:nightly AS builder
WORKDIR /benson
COPY . /benson

ENV RUST_VERSION nightly-2019-12-19
RUN apt-get update && \
    apt-get -y install apt-utils cmake pkg-config libssl-dev git clang libclang-dev && \
    rustup install $RUST_VERSION && \
    rustup default $RUST_VERSION && \
    rustup target add --toolchain $RUST_VERSION wasm32-unknown-unknown && \
    rustup target add --toolchain $RUST_VERSION x86_64-unknown-linux-musl && \
    mkdir -p /benson/.cargo
ENV CARGO_HOME=/benson/.cargo
RUN cargo build --release

FROM debian:stretch-slim
LABEL maintainer="ng8eke@163.com"

RUN apt-get update && \
    apt-get install -y ca-certificates openssl && \
    mkdir -p /root/.local/share/benson && \
    ln -s /root/.local/share/benson /data

COPY --from=0 /benson/target/release/benson /usr/local/bin
EXPOSE 30333 9933 9944
VOLUME ["/data"]
ENTRYPOINT ["/usr/local/bin/benson"]

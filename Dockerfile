FROM  rustlang/rust:nightly AS builder
WORKDIR /benson
COPY . /benson

ARG RUST_VERSION=1.47.0
ARG RUST_NIGHTLY=nightly-2020-11-18
RUN apt-get update && \
    apt-get -y install apt-utils cmake pkg-config libssl-dev git clang libclang-dev && \
    rustup uninstall nightly && \
    rustup install $RUST_VERSION && \
    rustup install $RUST_NIGHTLY && \
    rustup default $RUST_VERSION && \
    rustup target add --toolchain $RUST_NIGHTLY wasm32-unknown-unknown && \
    rustup target add --toolchain $RUST_VERSION x86_64-unknown-linux-musl && \
    mv /usr/local/rustup/toolchains/nightly* /usr/local/rustup/toolchains/nightly-x86_64-unknown-linux-gnu && \
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
# copy in genesis files
COPY --from=0 /benson/genesis /benson/genesis
# copy in wasm blob
COPY --from=0 /benson/target/release/wbuild/benson-runtime/benson_runtime.compact.wasm /benson
EXPOSE 30333 9933 9944
VOLUME ["/data"]
ENTRYPOINT ["/usr/local/bin/benson"]

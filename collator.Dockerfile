FROM rust:buster as factory

WORKDIR /factory

RUN apt-get update && \
    apt-get upgrade && \
    apt-get dist-upgrade -y -o Dpkg::Options::="--force-confnew" && \
    apt-get install -y cmake pkg-config libssl-dev git clang libclang-dev

RUN rustup default nightly-2022-06-16 && \
	rustup target add wasm32-unknown-unknown --toolchain nightly-2022-06-16

COPY . .

RUN cargo build --manifest-path ./node/parachain/Cargo.toml --release --locked

FROM phusion/baseimage:jammy-1.0.0

COPY --from=factory /factory/target/release/circuit-collator /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /node runner && \
    mkdir /node/data && \
    rm -rf /usr/lib/python* /usr/bin /usr/sbin /usr/share/man

USER runner

ENTRYPOINT ["/usr/local/bin/circuit-collator"]
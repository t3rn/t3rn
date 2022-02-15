FROM rust:buster as blacksmith

ARG BUILD_ARGS

WORKDIR /workshop

RUN rustup default nightly-2021-11-07 && \
	rustup target add wasm32-unknown-unknown --toolchain nightly-2021-11-07

RUN apt-get update && \
	apt-get dist-upgrade -y -o Dpkg::Options::="--force-confnew" && \
	apt-get install -y cmake pkg-config libssl-dev git clang libclang-dev

COPY ./substrate-parachain-template .

RUN cargo build --locked --release $BUILD_ARGS

###############################################################################

FROM phusion/baseimage:focal-1.1.0

COPY --from=blacksmith /workshop/target/release/parachain-collator /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /para para && \
    mkdir /para/data && \
    rm -rf /usr/lib/python* /usr/bin /usr/sbin /usr/share/man

USER para

VOLUME /para/data

EXPOSE 44444 8844 4499 44443 8843 4498

ENTRYPOINT ["/usr/local/bin/parachain-collator"]
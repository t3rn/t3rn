FROM rust:buster as blacksmith

ARG BRANCH=release-v0.9.17

WORKDIR /workshop

RUN rustup default nightly-2021-11-07 && \
	rustup target add wasm32-unknown-unknown --toolchain nightly-2021-11-07

ENV SCCACHE_BINARY=sccache-v0.2.15-x86_64-unknown-linux-musl.tar.gz
RUN curl -L -o ${SCCACHE_BINARY} https://github.com/mozilla/sccache/releases/download/v0.2.15/${SCCACHE_BINARY} && \
  tar -xvf ${SCCACHE_BINARY} && \
  chmod +x sccache-v0.2.15-x86_64-unknown-linux-musl/sccache && \
  mv sccache-v0.2.15-x86_64-unknown-linux-musl/sccache /usr/local/cargo/bin/sccache

ENV SCCACHE_CACHE_SIZE="10G"
ENV SCCACHE_DIR=/var/sccache
ENV RUSTC_WRAPPER="/usr/local/cargo/bin/sccache"

RUN --mount=type=cache,target=/var/cache/apt \
    apt-get update && \
	apt-get dist-upgrade -y -o Dpkg::Options::="--force-confnew" && \
	apt-get install -y cmake pkg-config libssl-dev git clang libclang-dev

RUN git clone \
        --depth 1 \
        --single-branch \
        --branch $BRANCH \
        https://github.com/paritytech/polkadot.git \
        .

RUN --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/var/sccache \
    cargo build --locked --release

###############################################################################

FROM phusion/baseimage:focal-1.1.0

COPY --from=blacksmith /workshop/target/release/polkadot /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /pdot pdot && \
    mkdir /pdot/data && \
    rm -rf /usr/lib/python* /usr/bin /usr/sbin /usr/share/man

USER pdot

VOLUME /pdot/data

EXPOSE 10001 8844 9944
     # 10002 8845 9945
     # 10003 8846 9946
     # 10004 8847 9947
     # 10005 8848 9948

ENTRYPOINT ["/usr/local/bin/polkadot"]
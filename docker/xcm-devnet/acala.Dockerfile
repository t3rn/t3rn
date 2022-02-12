FROM rust:buster as blacksmith

ARG ACALA_BRANCH
ARG BUILD_ARGS

WORKDIR /workshop

RUN rustup default nightly-2021-11-07 && \
	rustup target add wasm32-unknown-unknown --toolchain nightly-2021-11-07

RUN apt-get update && \
	apt-get dist-upgrade -y -o Dpkg::Options::="--force-confnew" && \
	apt-get install -y cmake pkg-config libssl-dev git clang libclang-dev

RUN git clone \
        --depth 1 \
        --single-branch \
        --branch ${ACALA_BRANCH:-release-acala-2.3.2} \
		--recurse-submodules \
		https://github.com/AcalaNetwork/Acala \
        .

RUN cargo build --locked --release $BUILD_ARGS

###############################################################################

FROM phusion/baseimage:focal-1.1.0

COPY --from=blacksmith /workshop/target/release/acala /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /acala acala && \
    mkdir /acala/data && \
    rm -rf /usr/lib/python* /usr/bin /usr/sbin /usr/share/man

USER acala

VOLUME /acala/data

EXPOSE 22222 8822 9922

ENTRYPOINT ["/usr/local/bin/acala"]
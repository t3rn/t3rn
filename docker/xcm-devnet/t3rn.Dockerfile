FROM rust:buster as blacksmith

ARG BRANCH=development

WORKDIR /workshop

RUN rustup default nightly-2021-11-07 && \
	rustup target add wasm32-unknown-unknown --toolchain nightly-2021-11-07

RUN apt-get update && \
	apt-get dist-upgrade -y -o Dpkg::Options::="--force-confnew" && \
	apt-get install -y cmake pkg-config libssl-dev git clang libclang-dev

# FIXME
COPY . .
# RUN	git clone \
# 		--depth 1 \
# 		--single-branch \
# 		--branch $BRANCH \
# 		--recurse-submodules \
# 		https://github.com/t3rn/x-t3rn.git \
# 		.

RUN cargo build --locked --release --features with-parachain-runtime

###############################################################################

FROM phusion/baseimage:focal-1.1.0

COPY --from=blacksmith /workshop/target/release/circuit-collator /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /t3rn t3rn && \
    mkdir /t3rn/data && \
    rm -rf /usr/lib/python* /usr/bin /usr/sbin /usr/share/man

USER t3rn

VOLUME /t3rn/data

EXPOSE 33333 8833 9933 33332 8832 9932

ENTRYPOINT ["/usr/local/bin/circuit-collator"]
FROM phusion/baseimage:jammy-1.0.0

ARG RELAYCHAIN_NAME=rococo
ARG PARACHAIN_NAME=t0rn

RUN useradd -m -u 1000 -U -s /bin/sh -d /node runner && \
    mkdir /node/data && \
    mkdir /node/keystore && \
    mkdir /node/specs

COPY --chown=runner target/release/circuit-collator /usr/local/bin/circuit-collator
COPY --chown=runner specs/$RELAYCHAIN_NAME.raw.json /node/specs/$RELAYCHAIN_NAME.raw.json
COPY --chown=runner specs/$PARACHAIN_NAME.raw.json /node/specs/$PARACHAIN_NAME.raw.json

VOLUME /node/data

USER runner

ENTRYPOINT ["/usr/local/bin/circuit-collator"]
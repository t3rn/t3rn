FROM phusion/baseimage:jammy-1.0.0

COPY --from=factory /factory/target/release/circuit-collator /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /node runner && \
    mkdir /node/data

COPY --chown=runner target/release/circuit-collator /usr/local/bin/circuit-collator

ENTRYPOINT ["/usr/local/bin/circuit-collator"]
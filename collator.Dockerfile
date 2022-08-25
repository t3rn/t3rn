FROM phusion/baseimage:jammy-1.0.0

RUN useradd -m -u 1000 -U -s /bin/sh -d /node runner && \
    mkdir /node/data

COPY --chown=runner target/release/circuit-collator /usr/local/bin/circuit-collator

VOLUME /node/data

USER runner

ENTRYPOINT ["/usr/local/bin/circuit-collator"]
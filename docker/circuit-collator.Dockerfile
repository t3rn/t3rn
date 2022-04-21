FROM phusion/baseimage:focal-1.1.0
LABEL maintainer="team@t3rn.io"
LABEL description="Circuit collator node"

RUN useradd -m -u 1000 -U -s /bin/sh -d /t3rn t3rn && \
    rm -rf /usr/lib/python* /usr/bin /usr/sbin /usr/share/man

COPY --chown=t3rn target/release/circuit-collator /usr/local/bin/circuit-collator

ENTRYPOINT ["/usr/local/bin/circuit-collator"]
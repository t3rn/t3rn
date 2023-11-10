FROM ubuntu:22.04
LABEL org.opencontainers.image.source https://github.com/t3rn/t3rn

ENV PARACHAIN_NAME=t2rn

RUN useradd -m -u 1000 -U -s /bin/sh -d /node runner && \
    mkdir -p /node/data /node/keystore /node/specs && \
    chown -R runner /node/data /node/keystore

COPY --chown=runner target/release/circuit-standalone /usr/local/bin/collator

USER runner
VOLUME /node/data
EXPOSE 33333 1933 1944 7003

ENTRYPOINT ["/usr/local/bin/collator"]
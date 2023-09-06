FROM ubuntu:22.04
LABEL org.opencontainers.image.source https://github.com/t3rn/t3rn

ENV RELAYCHAIN_NAME=polkadot
ENV PARACHAIN_NAME=t3rn

RUN useradd -m -u 1000 -U -s /bin/sh -d /node runner && \
    mkdir -p /node/data /node/keystore /node/specs && \
    chown -R runner /node/data /node/keystore

COPY target/release/${PARACHAIN_NAME}-collator /usr/local/bin/collator
COPY specs/$RELAYCHAIN_NAME.raw.json /node/specs/$RELAYCHAIN_NAME.raw.json
COPY specs/$PARACHAIN_NAME.raw.json /node/specs/$PARACHAIN_NAME.raw.json

USER runner
VOLUME /node/data
EXPOSE 33333 1933 1944 7003

ENTRYPOINT ["/usr/local/bin/collator"]
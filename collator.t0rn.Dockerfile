FROM ubuntu:22.04
LABEL org.opencontainers.image.source https://github.com/t3rn/t3rn

ENV RELAYCHAIN_NAME=rococo
ENV PARACHAIN_NAME=t0rn

RUN useradd -m -u 1000 -U -s /bin/sh -d /node runner && \
    mkdir -p /node/data /node/keystore /node/specs && \
    chown -R runner /node/data /node/keystore

COPY --chown=runner target/release/${PARACHAIN_NAME}-collator /usr/local/bin/collator
COPY --chown=runner specs/$RELAYCHAIN_NAME.raw.json /node/specs/$RELAYCHAIN_NAME.raw.json
COPY --chown=runner specs/$PARACHAIN_NAME.raw.json /node/specs/$PARACHAIN_NAME.raw.json

USER runner
VOLUME /node/data
EXPOSE 33333 1933 1944 7003

ENTRYPOINT ["/usr/local/bin/collator"]
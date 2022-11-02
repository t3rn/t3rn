FROM ubuntu:21.04
LABEL org.opencontainers.image.source https://github.com/t3rn/t3rn

ENV RELAYCHAIN_NAME=rococo
ENV PARACHAIN_NAME=t0rn

RUN useradd -m -u 1000 -U -s /bin/sh -d /node runner && \
    mkdir /node/data && \
    mkdir /node/keystore && \
    mkdir /node/specs

COPY --chown=runner target/release/${PARACHAIN_NAME}-collator /usr/local/bin/${PARACHAIN_NAME}-collator
COPY --chown=runner specs/$RELAYCHAIN_NAME.raw.json /node/specs/$RELAYCHAIN_NAME.raw.json
COPY --chown=runner specs/$PARACHAIN_NAME.raw.json /node/specs/$PARACHAIN_NAME.raw.json

USER runner
VOLUME /node/data
EXPOSE 33333 1833 1933 7003

ENTRYPOINT ["/usr/local/bin/${PARACHAIN_NAME}-collator"]
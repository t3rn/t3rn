FROM ubuntu:22.04
LABEL org.opencontainers.image.source https://github.com/t3rn/t3rn

# RUN cp /etc/apt/sources.list /etc/apt/sources.list.bak
# RUN sed -i -re 's/([a-z]{2}\.)?archive.ubuntu.com|security.ubuntu.com/old-releases.ubuntu.com/g' /etc/apt/sources.list

RUN apt-get update && \
	DEBIAN_FRONTEND=noninteractive apt-get install -y \
		libssl-dev \
		ca-certificates \
		curl && \
# apt cleanup
	apt-get autoremove -y && \
	apt-get clean && \
	find /var/lib/apt/lists/ -type f -not -name lock -delete

ENV RELAYCHAIN_NAME=rococo
ENV PARACHAIN_NAME=t0rn

RUN useradd -m -u 1000 -U -s /bin/sh -d /node runner && \
    mkdir /node/data && \
    mkdir /node/keystore && \
    mkdir /node/specs

COPY --chown=runner target/release/circuit-collator /usr/local/bin/circuit-collator
RUN ls /usr/local/bin
COPY --chown=runner specs/$RELAYCHAIN_NAME.raw.json /node/specs/$RELAYCHAIN_NAME.raw.json
COPY --chown=runner specs/$PARACHAIN_NAME.raw.json /node/specs/$PARACHAIN_NAME.raw.json

USER runner
RUN mkdir /node/data2

RUN /usr/local/bin/circuit-collator --version

EXPOSE 33333 1833 1933 7003

ENTRYPOINT ["/usr/local/bin/circuit-collator"]
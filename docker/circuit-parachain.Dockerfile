FROM ubuntu:20.04
LABEL maintainer="team@t3rn.io"
LABEL description="Circuit parachain Collator node"
ARG PROFILE=release

RUN useradd -m -u 1000 -U -s /bin/sh -d /t3rn t3rn && \
	mkdir -p /t3rn/.local/share/circuit && \
	chown -R t3rn:t3rn /t3rn && \
	ln -s /t3rn/.local/share/circuit /data

USER t3rn

COPY --chown=t3rn circuit-parachain/target/release /t3rn
RUN chmod uog+x /t3rn/parachain-collator

# 9933 for RPC call
# 9944 for Websocket
EXPOSE 9933 9944

VOLUME ["/data"]

CMD ["/t3rn/parachain-collator"]

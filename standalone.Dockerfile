FROM docker.io/paritytech/ci-linux:production AS build

ADD /finality-verifiers /finality-verifiers
ADD /node/standalone /node/standalone
ADD /pallets /pallets
ADD /primitives /primitives
ADD /protocol /protocol
ADD /relayers /relayers
ADD /runtime /runtime
ADD /types /types

WORKDIR /node/standalone

RUN apt-get clean && apt update && apt install -y build-essential cmake
RUN cargo fetch
RUN rustup install nightly-2022-06-16
RUN rustup target add wasm32-unknown-unknown --toolchain nightly-2022-06-16
RUN cargo +nightly-2022-06-16 build

# Build clean circuit-standalone image
FROM docker.io/paritytech/ci-linux:production

RUN mkdir /app
WORKDIR /app

COPY --from=build /node/standalone/target/debug/circuit-standalone /app/

CMD ["/app/circuit-standalone", "--dev", "-lruntime=debug"]

#!/bin/bash -x

git clone \
    --depth 1 \
    --single-branch \
    --branch release-v0.9.13 \
    https://github.com/paritytech/polkadot.git

git clone \
    --depth 1 \
    --single-branch \
    --branch release-acala-2.3.2 \
    --recurse-submodules \
    https://github.com/AcalaNetwork/Acala acala

git clone \
    --depth 1 \
    --single-branch \
    --branch development \
    --recurse-submodules \
    https://github.com/t3rn/x-t3rn

cargo build \
    --manifest-path ./polkadot/Cargo.toml \
    --release \
    --locked

cargo build \
    --manifest-path ./acala/Cargo.toml \
    --release \
    --locked \
    --features with-acala-runtime

cargo build \
    --manifest-path ./x-t3rn/Cargo.toml \
    --release \
    --features with-parachain-runtime

mkdir ./specs

./polkadot/target/release/polkadot build-spec \
    --chain rococo-local \
    --disable-default-bootnode \
> ./specs/rococo-local.json

sed -i 's/"nextFreeParaId": [[:digit:]]\+/"nextFreeParaId": 4000/g' \
    ./specs/rococo-local.json

./polkadot/target/release/polkadot build-spec \
    --chain ./specs/rococo-local.json \
    --disable-default-bootnode \
    --raw \
> ./specs/rococo-local.raw.json

./acala/target/release/acala build-spec \
    --chain local \
    --disable-default-bootnode \
> ./specs/acala.json

sed -i 's/"parachainId": [[:digit:]]\+/"parachainId": 2000/g' ./specs/acala.json

./acala/target/release/acala build-spec \
    --chain ./specs/acala.json \
    --disable-default-bootnode \
    --raw \
> ./specs/acala.raw.json

./x-t3rn/target/release/circuit-collator build-spec \
    --disable-default-bootnode \
> ./specs/t3rn.json

sed -i 's/"parachainId": [[:digit:]]\+/"parachainId": 3000/g' ./specs/t3rn.json

./x-t3rn/target/release/circuit-collator build-spec \
    --chain ./specs/t3rn.json \
    --disable-default-bootnode \
    --raw \
> ./specs/t3rn.raw.json

./acala/target/release/acala export-genesis-state \
    --chain ./specs/acala.raw.json \
> ./specs/acala.genesis

./x-t3rn/target/release/circuit-collator export-genesis-state \
    --chain ./specs/t3rn.raw.json \
> ./specs/t3rn.genesis

./acala/target/release/acala export-genesis-wasm \
    --chain ./specs/acala.raw.json \
> ./specs/acala.wasm

./x-t3rn/target/release/circuit-collator export-genesis-wasm \
> ./specs/t3rn.wasm

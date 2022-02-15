#!/bin/bash -x

docker build -t polkadot:release-v0.9.16 -f polkadot.Dockerfile .
docker build -t acala:release-acala-2.3.2 -f acala.Dockerfile .
docker build -t circuit-collator:latest -f t3rn.Dockerfile .
docker build -t parachain-collator:latest -f parachain.Dockerfile .

mkdir ./specs

docker run polkadot build-spec \
    --chain rococo-local \
    --disable-default-bootnode \
> ./specs/rococo-local.json

sed -i 's/"nextFreeParaId": [[:digit:]]\+/"nextFreeParaId": 5000/g' \
    ./specs/rococo-local.json

docker run polkadot build-spec \
    --chain ./specs/rococo-local.json \
    --disable-default-bootnode \
    --raw \
> ./specs/rococo-local.raw.json

docker run acala build-spec \
    --chain local \
    --disable-default-bootnode \
> ./specs/acala.json

sed -i 's/"parachainId": [[:digit:]]\+/"parachainId": 2000/g' ./specs/acala.json

docker run acala build-spec \
    --chain ./specs/acala.json \
    --disable-default-bootnode \
    --raw \
> ./specs/acala.raw.json

docker run circuit-collator build-spec \
    --disable-default-bootnode \
> ./specs/t3rn.json

sed -i 's/"parachainId": [[:digit:]]\+/"parachainId": 3000/g' ./specs/t3rn.json

docker run circuit-collator build-spec \
    --chain ./specs/t3rn.json \
    --disable-default-bootnode \
    --raw \
> ./specs/t3rn.raw.json

docker run parachain-collator build-spec \
    --disable-default-bootnode \
> ./specs/parachain.json

sed -i 's/"parachainId": [[:digit:]]\+/"parachainId": 4000/g' ./specs/parachain.json

docker run parachain-collator build-spec \
    --chain ./specs/parachain.json \
    --disable-default-bootnode \
    --raw \
> ./specs/parachain.raw.json

docker run acala export-genesis-state \
    --chain ./specs/acala.raw.json \
> ./specs/acala.genesis

docker run circuit-collator export-genesis-state \
    --chain ./specs/t3rn.raw.json \
> ./specs/t3rn.genesis

docker run parachain-collator export-genesis-state \
    --chain ./specs/parachain.raw.json \
> ./specs/parachain.genesis

docker run parachain-collator export-genesis-wasm \
    --chain ./specs/parachain.raw.json \
> ./specs/parachain.wasm

docker run acala export-genesis-wasm \
    --chain ./specs/acala.raw.json \
> ./specs/acala.wasm

docker run circuit-collator export-genesis-wasm \
> ./specs/t3rn.wasm

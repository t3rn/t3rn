#!/bin/bash -xEe

# NOTE: these tags should stay in sync with those in docker-compose.yml
docker build -t polkadot:release-v0.9.13 -f polkadot.Dockerfile .
docker build -t acala:release-acala-2.2.0 -f acala.Dockerfile .
docker build -t circuit-collator:latest -f t3rn.Dockerfile ../..
docker build -t parachain-collator:polkadot-v0.9.13 -f pchain.Dockerfile .

mkdir -p ./specs

docker run \
    polkadot:release-v0.9.13 \
    build-spec \
    --chain rococo-local \
    --disable-default-bootnode \
> ./specs/rococo-local.json

sed -i 's/"nextFreeParaId": [[:digit:]]\+/"nextFreeParaId": 5000/g' \
    ./specs/rococo-local.json

docker run \
    -v "$(pwd)/specs:/usr/local/etc" \
    polkadot:release-v0.9.13 \
    build-spec \
    --chain /usr/local/etc/rococo-local.json \
    --disable-default-bootnode \
    --raw \
> ./specs/rococo-local.raw.json

docker run acala:release-acala-2.2.0 build-spec \
    --chain local \
    --disable-default-bootnode \
> ./specs/acala.json

sed -i 's/"parachainId": [[:digit:]]\+/"parachainId": 2000/g' ./specs/acala.json

docker run \
    -v "$(pwd)/specs:/usr/local/etc" \
    acala:release-acala-2.2.0 \
    build-spec \
    --chain /usr/local/etc/acala.json \
    --disable-default-bootnode \
    --raw \
> ./specs/acala.raw.json

docker run circuit-collator:latest build-spec \
    --disable-default-bootnode \
> ./specs/t3rn.json

sed -i 's/"parachainId": [[:digit:]]\+/"parachainId": 3000/g' ./specs/t3rn.json

docker run \
    -v "$(pwd)/specs:/usr/local/etc" \
    circuit-collator:latest \
    build-spec \
    --chain /usr/local/etc/t3rn.json \
    --disable-default-bootnode \
    --raw \
> ./specs/t3rn.raw.json

docker run parachain-collator:latest build-spec \
    --disable-default-bootnode \
> ./specs/pchain.json

sed -i 's/"parachainId": [[:digit:]]\+/"parachainId": 4000/g' ./specs/pchain.json

docker run \
    -v "$(pwd)/specs:/usr/local/etc" \
    parachain-collator:polkadot-v0.9.13 \
    build-spec \
    --chain /usr/local/etc/pchain.json \
    --disable-default-bootnode \
    --raw \
> ./specs/pchain.raw.json

docker run \
    -v "$(pwd)/specs:/usr/local/etc" \
    acala:release-acala-2.2.0 \
    export-genesis-state \
    --chain /usr/local/etc/acala.raw.json \
> ./specs/acala.genesis

docker run \
    -v "$(pwd)/specs:/usr/local/etc" \
    circuit-collator:latest \
    export-genesis-state \
    --chain /usr/local/etc/t3rn.raw.json \
> ./specs/t3rn.genesis

docker run \
    -v "$(pwd)/specs:/usr/local/etc" \
    parachain-collator:polkadot-v0.9.13 \
    export-genesis-state \
    --chain /usr/local/etc/pchain.raw.json \
> ./specs/pchain.genesis

docker run \
    -v "$(pwd)/specs:/usr/local/etc" \
    parachain-collator:polkadot-v0.9.13 \
    export-genesis-wasm \
    --chain /usr/local/etc/pchain.raw.json \
> ./specs/pchain.wasm

docker run \
    -v "$(pwd)/specs:/usr/local/etc" \
    acala:release-acala-2.2.0 \
    export-genesis-wasm \
    --chain /usr/local/etc/acala.raw.json \
> ./specs/acala.wasm

docker run circuit-collator:latest export-genesis-wasm \
> ./specs/t3rn.wasm

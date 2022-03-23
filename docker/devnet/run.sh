#!/bin/bash

set -xEeo pipefail

build_docker_images() {
  # NOTE: docker tags should stay in sync with those in docker-compose.yml
  if ! docker inspect polkadot:release-v0.9.17 > /dev/null; then
    DOCKER_BUILDKIT=1 docker build \
      -t polkadot:release-v0.9.17 \
      -f polkadot.Dockerfile .
  fi
  if ! docker inspect circuit-collator:update_v0.9.17 > /dev/null; then
    DOCKER_BUILDKIT=1 docker build \
      -t circuit-collator:update_v0.9.17 \
      -f t3rn.Dockerfile ../..
  fi
  # if ! docker inspect acala:release-acala-2.4.0 > /dev/null; then
  #   DOCKER_BUILDKIT=1 docker build \
  #     -t acala:release-acala-2.4.0 \
  #     -f acala.Dockerfile .
  # fi
}

keygen() {
  ## gen custom node keys 4 the 2 parachains
  subkey generate --scheme Sr25519 > ./specs/t3rn1.key
  subkey generate --scheme Sr25519 > ./specs/t3rn2.key
  # subkey generate --scheme Sr25519 > ./specs/acala1.key
  # subkey generate --scheme Sr25519 > ./specs/acala2.key
}

build_relay_chain_spec() {
  docker run \
      polkadot:release-v0.9.17 \
      build-spec \
      --chain rococo-local \
      --disable-default-bootnode \
  > ./specs/rococo-local.json
  sed 's/"nextFreeParaId": [[:digit:]]\+/"nextFreeParaId": 3333/g' \
      -i ./specs/rococo-local.json
  docker run \
      -v "$(pwd)/specs:/usr/local/etc" \
      polkadot:release-v0.9.17 \
      build-spec \
      --chain /usr/local/etc/rococo-local.json \
      --disable-default-bootnode \
      --raw \
  > ./specs/rococo-local.raw.json
}

build_para_chain_specs() {
  # NOTE: included parachain ids should stay in sync with those in README.md
  t3rn1_adrs=$(grep -oP '(?<=\(SS58\):\s)[^\n]+' ./specs/t3rn1.key)
  t3rn2_adrs=$(grep -oP '(?<=\(SS58\):\s)[^\n]+' ./specs/t3rn2.key)
  acala1_adrs=$(grep -oP '(?<=\(SS58\):\s)[^\n]+' ./specs/acala1.key)
  acala2_adrs=$(grep -oP '(?<=\(SS58\):\s)[^\n]+' ./specs/acala2.key)
  ## gen t3rn chain spec
  docker run circuit-collator:update_v0.9.17 build-spec \
      --disable-default-bootnode \
  > ./specs/t3rn.json
  # set parachain id(s)
  sed 's/"paraId": [[:digit:]]\+/"paraId": 3333/g' \
      -i ./specs/t3rn.json
  sed 's/"para_id": [[:digit:]]\+/"para_id": 3333/g' \
      -i ./specs/t3rn.json
  sed 's/"parachainId": [[:digit:]]\+/"parachainId": 3333/g' \
      -i ./specs/t3rn.json
  # set the t3rn1 node address - replacing alice
  sed "s/5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY/$t3rn1_adrs/g" \
      -i ./specs/t3rn.json
  # set the t3rn2 node address - replacing bob
  sed "s/5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty/$t3rn2_adrs/g" \
      -i ./specs/t3rn.json
  docker run \
      -v "$(pwd)/specs:/usr/local/etc" \
      circuit-collator:update_v0.9.17 \
      build-spec \
      --chain /usr/local/etc/t3rn.json \
      --disable-default-bootnode \
      --raw \
  > ./specs/t3rn.raw.json
  # ## gen acala chain spec
  # docker run acala:release-acala-2.4.0 build-spec \
  #     --disable-default-bootnode \
  #     --chain acala
  # > ./specs/acala.json
  # # set parachain id(s)
  # sed 's/"paraId": [[:digit:]]\+/"paraId": 3334/g' \
  #     -i ./specs/acala.json
  # sed 's/"para_id": [[:digit:]]\+/"para_id": 3334/g' \
  #     -i ./specs/acala.json
  # sed 's/"parachainId": [[:digit:]]\+/"parachainId": 3334/g' \
  #     -i ./specs/acala.json
  # # set the acala1 node address
  # sed "s/5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY/$acala1_adrs/g" \
  #     -i ./specs/acala.json
  # # set the acala2 node address
  # sed "s/5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty/$acala2_adrs/g" \
  #     -i ./specs/acala.json
  # docker run \
  #     -v "$(pwd)/specs:/usr/local/etc" \
  #     acala:release-acala-2.4.0 \
  #     build-spec \
  #     --chain /usr/local/etc/acala.json \
  #     --disable-default-bootnode \
  #     --raw \
  # > ./specs/acala.raw.json
}

build_para_genesis_states() {
  docker run \
      -v "$(pwd)/specs:/usr/local/etc" \
      circuit-collator:update_v0.9.17 \
      export-genesis-state \
      --chain /usr/local/etc/t3rn.raw.json \
  > ./specs/t3rn.genesis
  # docker run \
  #     -v "$(pwd)/specs:/usr/local/etc" \
  #     acala:release-acala-2.4.0 \
  #     export-genesis-state \
  #     --chain /usr/local/etc/acala.raw.json \
  # > ./specs/acala.genesis
}

build_para_wasm_runtimes() {
  # docker run \
  #     -v "$(pwd)/specs:/usr/local/etc" \
  #     acala:release-acala-2.4.0 \
  #     export-genesis-wasm \
  #     --chain /usr/local/etc/acala.raw.json \
  # > ./specs/acala.wasm
  docker run circuit-collator:update_v0.9.17 export-genesis-wasm \
  > ./specs/t3rn.wasm
}

set_keys() {
  t3rn1_phrase="$(grep -oP '(?<=phrase:)[^\n]+' ./specs/t3rn1.key | xargs)"
  t3rn2_phrase="$(grep -oP '(?<=phrase:)[^\n]+' ./specs/t3rn2.key | xargs)"
  # acala1_phrase="$(grep -oP '(?<=phrase:)[^\n]+' ./specs/acala1.key | xargs)"
  # acala2_phrase="$(grep -oP '(?<=phrase:)[^\n]+' ./specs/acala2.key | xargs)"
  docker exec \
    -u t3rn \
    t3rn1 \
    circuit-collator \
    key \
    insert \
    --base-path /t3rn/data \
    --chain /t3rn/t3rn.raw.json \
    --scheme Sr25519 \
    --suri "$t3rn1_phrase" \
    --key-type aura
  docker exec \
    -u t3rn \
    t3rn2 \
    circuit-collator \
    key \
    insert \
    --base-path /t3rn/data \
    --chain /t3rn/t3rn.raw.json \
    --scheme Sr25519 \
    --suri "$t3rn2_phrase" \
    --key-type aura
  # docker exec \
  #   -u acala \
  #   acala1 \
  #   acala \
  #   key \
  #   insert \
  #   --base-path /acala/data \
  #   --chain /acala/acala.raw.json \
  #   --scheme Sr25519 \
  #   --suri "$acala1_phrase" \
  #   --key-type aura
  # docker exec \
  #   -u acala \
  #   acala2 \
  #   acala\
  #   key \
  #   insert \
  #   --base-path /acala/data \
  #   --chain /acala/acala.raw.json \
  #   --scheme Sr25519 \
  #   --suri "$acala2_phrase" \
  #   --key-type aura
}

onboard() {
  npx --yes @polkadot/api-cli@beta \
    --ws 'ws://localhost:9944' \
    --seed '//Alice' \
    tx.registrar.reserve

  printf \
    "%d {\"genesisHead\":\"%s\",\"validationCode\":\"%s\",\"parachain\":true}" \
    3333 \
    $(<./specs/t3rn.genesis) \
    $(<./specs/t3rn.wasm) \
    > /tmp/t3rn.params

  npx @polkadot/api-cli@beta \
    --ws 'ws://localhost:9944' \
    --sudo \
    --seed '//Alice' \
    --params /tmp/t3rn.params \
    tx.parasSudoWrapper.sudoScheduleParaInitialize

  # npx @polkadot/api-cli@beta \
  #   --ws 'ws://localhost:9944' \
  #   --seed '//Alice' \
  #   tx.registrar.reserve

  # printf \
  #   "%d {\"genesisHead\":\"%s\",\"validationCode\":\"%s\",\"parachain\":true}" \
  #   3334 \
  #   $(<./specs/acala.genesis) \
  #   $(<./specs/acala.wasm) \
  #   > /tmp/acala.params

  # npx @polkadot/api-cli@beta \
  #   --ws 'ws://localhost:9944' \
  #   --sudo \
  #   --seed '//Alice' \
  #   --params /tmp/acala.params \
  #   tx.parasSudoWrapper.sudoScheduleParaInitialize

  #   rm /tmp/{acala.params,t3rn.params}
}

case ${1:-devnet} in
devnet|dev|net)
  mkdir -p ./data/{alice,bob,charlie,dave,eve,t3rn1,t3rn2,acala1,acala2}
  docker-compose up > /dev/null &
  sleep 13s # allow node startup ~ basepath/datadir/keystore creation
  echo "⛓️ setting up collator keystores and initializing parachain onboarding..."
  set_keys
  onboard
  echo "👀 parachains onboarding => https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/parachains"
  ;;
setkeys|keys)
  set_keys
  ;;
onboard|board)
  onboard
  ;;
clean|cleanup)
  docker-compose down
  rm -r ./data/{alice,bob,charlie,dave,eve,t3rn1,t3rn2,acala1,acala2}/*
  ;;
build|make|mk)
  mkdir -p ./specs
  build_docker_images
  keygen
  build_relay_chain_spec
  build_para_chain_specs
  build_para_genesis_states
  build_para_wasm_runtimes
  ;;
*)
  echo "unknown subcommand" 1>&2
  exit 1
  ;;
esac
#!/bin/bash

set -xEeo pipefail

PDOT_BRANCH=${PDOT_BRANCH:-release-v0.9.27}

dir=$(git rev-parse --show-toplevel)/devnet

build_nodes() {
  if [[ ! -x $root_dir/devnet/bin/polkadot ]]; then
    d=$(mktemp -d)
    git clone \
      --depth 1 \
      --branch $PDOT_BRANCH \
      https://github.com/paritytech/polkadot.git \
      $d
    cargo build --manifest-path $d/Cargo.toml --release --locked
    cp $d/target/release/polkadot $root_dir/devnet/bin/polkadot
    rm -rf $d
  fi
  cargo build \
    --manifest-path $root_dir/node/t3rn-parachain/Cargo.toml \
    --release \
    --locked
  cp \
    $root_dir/target/release/circuit-collator \
    $root_dir/devnet/bin/devnet-circuit-collator
}

keygen() {
  if ! cargo install --list | grep -Fq subkey; then
    echo "installing subkey..."
    cargo install subkey \
      --git https://github.com/paritytech/substrate \
      --force
  fi
  ## gen custom node keys 4 the 2 parachains
  subkey generate --scheme Sr25519 --network t3rn > $dir/specs/circuita1.key
  subkey generate --scheme Sr25519 --network t3rn > $dir/specs/circuita2.key
  subkey generate --scheme Sr25519 --network t3rn > $dir/specs/circuitb1.key
  subkey generate --scheme Sr25519 --network t3rn > $dir/specs/circuitb2.key
}

build_relay_chain_spec() {
  echo "build_relay_chain_spec"
  $dir/bin/polkadot \
      build-spec \
      --chain polkadot-local \
  > $dir/specs/polkadot-local.json

  $dir/bin/polkadot \
      build-spec \
      --chain $dir/specs/polkadot-local.json \
      --raw \
  > $dir/specs/polkadot-local.raw.json
  sed 's/"nextFreeParaId": [[:digit:]]\+/"nextFreeParaId": 3333/g' \
    -i $dir/specs/polkadot-local.json
}

build_para_chain_specs() {
  circuita1_adrs=$(grep -oP '(?<=\(SS58\):\s)[^\n]+' $dir/specs/circuita1.key)
  circuita2_adrs=$(grep -oP '(?<=\(SS58\):\s)[^\n]+' $dir/specs/circuita2.key)
  circuitb1_adrs=$(grep -oP '(?<=\(SS58\):\s)[^\n]+' $dir/specs/circuitb1.key)
  circuitb2_adrs=$(grep -oP '(?<=\(SS58\):\s)[^\n]+' $dir/specs/circuitb2.key)
  ## gen t3rn chain spec -- A
  $dir/bin/devnet-circuit-collator build-spec \
      --disable-default-bootnode --chain polkadot \
  > $dir/specs/circuita.json

  ## gen t3rn chain spec -- B
  ## gen pchain chain spec
  $dir/bin/devnet-circuit-collator build-spec \
      --disable-default-bootnode --chain polkadot \
  > $dir/specs/circuitb.json

  # set parachain id(s)
  sed 's/"paraId": [[:digit:]]\+/"paraId": 3333/g' \
      -i $dir/specs/circuita.json
  sed 's/"para_id": [[:digit:]]\+/"para_id": 3333/g' \
      -i $dir/specs/circuita.json
  sed 's/"parachainId": [[:digit:]]\+/"parachainId": 3333/g' \
      -i $dir/specs/circuita.json
  # set the circuita1 node address - replacing alice
  sed "s/t3W7yG2pkGdLogoX6KJm5KtPMMWBQygvcZArcjtjo5AsJPad2/$circuita1_adrs/g" \
      -i $dir/specs/circuita.json
  # set the circuita2 node address - replacing bob
  sed "s/t3X7yGXEmCwTwwS6aFwwNeXDrGT2EU9Cy13G4qUPNpVh4Phjm/$circuita2_adrs/g" \
      -i $dir/specs/circuita.json
  # reset sudo
#  sed 's/"key": \"[0-9a-zA-Z]\+\"/"key":\"5DvUtHDPScgGFfWuK6RMczuLsHMJsmFRoVJYd6KCkseN72RK\"/' \
#    -i $dir/specs/circuita.json
#  # regrant alice some balance - taking from charlie
#  sed 's/5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y/5DvUtHDPScgGFfWuK6RMczuLsHMJsmFRoVJYd6KCkseN72RK/' \
#    -i $dir/specs/circuita.json

  # set parachain id(s)
  sed 's/"paraId": [[:digit:]]\+/"paraId": 3334/g' \
      -i $dir/specs/circuitb.json
  sed 's/"para_id": [[:digit:]]\+/"para_id": 3334/g' \
      -i $dir/specs/circuitb.json
  sed 's/"parachainId": [[:digit:]]\+/"parachainId": 3334/g' \
      -i $dir/specs/circuitb.json
  # set the circuitb1 node address - replacing alice
  sed "s/t3W7yG2pkGdLogoX6KJm5KtPMMWBQygvcZArcjtjo5AsJPad2/$circuitb1_adrs/g" \
      -i $dir/specs/circuitb.json
  # set the circuitb2 node address - replacing bob
  sed "s/t3X7yGXEmCwTwwS6aFwwNeXDrGT2EU9Cy13G4qUPNpVh4Phjm/$circuitb2_adrs/g" \
      -i $dir/specs/circuitb.json
  # reset alice to sudo
#  sed 's/"key": \"[0-9a-zA-Z]\+\"/"key":\"5DvUtHDPScgGFfWuK6RMczuLsHMJsmFRoVJYd6KCkseN72RK\"/' \
#    -i $dir/specs/circuitb.json
  # regrant alice some balance - taking from charlie
#  sed 's/5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y/5DvUtHDPScgGFfWuK6RMczuLsHMJsmFRoVJYd6KCkseN72RK/' \
#    -i $dir/specs/circuitb.json

  $dir/bin/devnet-circuit-collator \
      build-spec \
      --chain $dir/specs/circuita.json \
      --disable-default-bootnode \
      --raw \
  > $dir/specs/circuita.raw.json

  $dir/bin/devnet-circuit-collator \
      build-spec \
      --chain $dir/specs/circuitb.json \
      --disable-default-bootnode \
      --raw \
  > $dir/specs/circuitb.raw.json
}

build_para_genesis_states() {
  $dir/bin/devnet-circuit-collator \
      export-genesis-state \
      --chain $dir/specs/circuita.raw.json \
  > $dir/specs/circuita.genesis
  $dir/bin/devnet-circuit-collator \
      export-genesis-state \
      --chain $dir/specs/circuitb.raw.json \
  > $dir/specs/circuitb.genesis
}

build_para_wasm_runtimes() {
  $dir/bin/devnet-circuit-collator export-genesis-wasm \
          --chain $dir/specs/circuita.raw.json \
  > $dir/specs/circuita.wasm
  $dir/bin/devnet-circuit-collator export-genesis-wasm \
          --chain $dir/specs/circuitb.raw.json \
  > $dir/specs/circuitb.wasm
}

set_keys() {
  circuita1_phrase="$(grep -oP '(?<=phrase:)[^\n]+' $dir/specs/circuita1.key | xargs)"
  circuita2_phrase="$(grep -oP '(?<=phrase:)[^\n]+' $dir/specs/circuita2.key | xargs)"
  circuitb1_phrase="$(grep -oP '(?<=phrase:)[^\n]+' $dir/specs/circuitb1.key | xargs)"
  circuitb2_phrase="$(grep -oP '(?<=phrase:)[^\n]+' $dir/specs/circuitb2.key | xargs)"
  $dir/bin/devnet-circuit-collator \
    --base-path $dir/data/circuita1 \
    --chain $dir/specs/circuita.raw.json \
    --keystore-path $dir/specs/circuita1.key
  $dir/bin/devnet-circuit-collator \
    --base-path $dir/data/circuita2 \
    --chain $dir/specs/circuita.raw.json \
    --keystore-path $dir/specs/circuita2.key
  $dir/bin/devnet-circuit-collator \
    --base-path $dir/data/circuitb1 \
    --chain $dir/specs/circuitb.raw.json \
    --keystore-path $dir/specs/circuitb1.key
  $dir/bin/devnet-circuit-collator \
    --base-path $dir/data/circuitb2 \
    --chain $dir/specs/circuitb.raw.json \
    --keystore-path $dir/specs/circuitb2.key
}

onboard() {
  d=$(mktemp -d)
  npx --yes @polkadot/api-cli@beta \
    --ws ws://localhost:1944 \
    --seed //Alice \
    tx.registrar.reserve
  printf \
    "%d {\"genesisHead\":\"%s\",\"validationCode\":\"%s\",\"parachain\":true}" \
    3333 \
    $(<$dir/specs/circuita.genesis) \
    $(<$dir/specs/circuita.wasm) \
  > $d/circuita.params
  npx --yes @polkadot/api-cli@beta \
    --ws ws://localhost:1944 \
    --sudo \
    --seed //Alice \
    --params $d/circuita.params \
    tx.parasSudoWrapper.sudoScheduleParaInitialize
  npx --yes @polkadot/api-cli@beta \
    --ws ws://localhost:1944 \
    --seed //Alice \
    tx.registrar.reserve
  printf \
    "%d {\"genesisHead\":\"%s\",\"validationCode\":\"%s\",\"parachain\":true}" \
    3334 \
    $(<$dir/specs/circuitb.genesis) \
    $(<$dir/specs/circuitb.wasm) \
  > $d/circuitb.params
  npx --yes @polkadot/api-cli@beta \
    --ws ws://localhost:1944 \
    --sudo \
    --seed //Alice \
    --params $d/circuitb.params \
    tx.parasSudoWrapper.sudoScheduleParaInitialize
}

start_nodes() {

  echo "hello start_nodes"
  if [ "$(uname)" == "Darwin" ]; then
    term_name=iTerm
  else
    term_name=gnome-terminal
  fi
  if ! npm ls --global | grep -qF ttab; then
    npm i -g ttab
  fi
  nohup $dir/bin/polkadot \
    --ws-port 1944 \
    --alice \
    --validator \
    --tmp \
    --rpc-cors all \
    --unsafe-ws-external \
    --unsafe-rpc-external \
    --chain $dir/specs/polkadot-local.raw.json > $dir/logs/relay.alice.out 2>&1 &
  nohup $dir/bin/polkadot \
    --bob \
    --validator \
    --tmp \
    --chain $dir/specs/polkadot-local.raw.json > $dir/logs/relay.bob.out 2>&1 &
  nohup $dir/bin/polkadot \
    --charlie \
    --validator \
    --tmp \
    --rpc-cors all \
    --unsafe-ws-external \
    --unsafe-rpc-external \
    --chain $dir/specs/polkadot-local.raw.json > $dir/logs/relay.charlie.out 2>&1 &
  nohup $dir/bin/polkadot \
    --dave \
    --validator \
    --tmp \
    --chain $dir/specs/polkadot-local.raw.json > $dir/logs/relay.dave.out 2>&1 &
  nohup $dir/bin/polkadot \
    --eve \
    --validator \
    --tmp \
    --chain $dir/specs/polkadot-local.raw.json > $dir/logs/relay.eve.out 2>&1 &



#  grep -oP '(?<=seed:       0x)[^\n]+' specs/circuitb2.key
  circuita1_seed=$(grep -oP '(?<=seed:       0x)[^\n]+' $dir/specs/circuita1.key)
  circuita2_seed=$(grep -oP '(?<=seed:       0x)[^\n]+' $dir/specs/circuita2.key)
  circuitb1_seed=$(grep -oP '(?<=seed:       0x)[^\n]+' $dir/specs/circuitb1.key)
  circuitb2_seed=$(grep -oP '(?<=seed:       0x)[^\n]+' $dir/specs/circuitb2.key)

  nohup $dir/bin/devnet-circuit-collator \
    --port 33333 \
    --ws-port 1933 \
    --rpc-port 1833 \
    --collator \
    --base-path $dir/data/circuita1 \
    --node-key "$circuita1_seed" \
    --rpc-cors all \
    --unsafe-ws-external \
    --unsafe-rpc-external \
    --chain $dir/specs/circuita.raw.json \
    --execution Wasm \
    -- \
    --chain $dir/specs/polkadot-local.raw.json \
    --discover-local > $dir/logs/circuit.a.out 2>&1 &
  nohup $dir/bin/devnet-circuit-collator \
    --port 33332 \
    --ws-port 1932 \
    --rpc-port 1832 \
    --collator \
    --base-path $dir/data/circuita2 \
    --node-key "$circuita2_seed" \
    --chain $dir/specs/circuita.raw.json \
    --execution Wasm \
    -- \
    --chain $dir/specs/polkadot-local.raw.json \
    --discover-local > $dir/logs/circuit.b.out 2>&1 &
  nohup $dir/bin/devnet-circuit-collator \
    --port 23333 \
    --ws-port 2933 \
    --rpc-port 2833 \
    --collator \
    --base-path $dir/data/circuitb1 \
    --node-key "$circuitb1_seed" \
    --chain $dir/specs/circuitb.raw.json \
    --execution Wasm \
    -- \
    --chain $dir/specs/polkadot-local.raw.json \
    --discover-local > $dir/logs/circuit.c.out 2>&1 &
  nohup $dir/bin/devnet-circuit-collator \
    --port 23332 \
    --ws-port 2932 \
    --rpc-port 2832 \
    --collator \
    --base-path $dir/data/circuitb2 \
    --node-key "$circuitb2_seed" \
    --chain $dir/specs/circuitb.raw.json \
    --execution Wasm \
    -- \
    --chain $dir/specs/polkadot-local.raw.json \
    --discover-local > $dir/logs/circuit.d.out 2>&1 &
}

devnet() {
  root_dir=$(git rev-parse --show-toplevel)
  mkdir -p $root_dir/devnet/{bin,data,specs}
  mkdir -p $root_dir/devnet/data/{circuita1,circuita2,circuitb1,circuitb2}
  build_nodes
  keygen
  build_relay_chain_spec
  build_para_chain_specs
  build_para_genesis_states
  build_para_wasm_runtimes
  rm -rf $dir/data/*
  start_nodes
  npx --yes wait-port -t 33000 localhost:1933
  npx --yes wait-port -t 33000 localhost:1944
#  set_keys
  onboard
}

teardown() {
  rm -rf $dir/data/*
  set +Ee
  killall polkadot
  killall devnet-circuit-collator
  set -Ee
}

main() {
  case ${1:-spinup} in
  spinup|up)
    echo "spinup" 1>&2
    devnet
    ;;
  teardown|down)
    echo "teardown" 1>&2
    teardown
    ;;
  *)
    echo "usg: $dir/run.sh [up|down]" 1>&2
    exit 1
    ;;
  esac
}

main "$@"
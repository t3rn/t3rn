#!/bin/bash

set -xEe

if [[ -z $1 || -z "$2" || -z "$3" ]]; then
  echo "usage: $0 \$tag \"'t0rn collator one secret'\" \"'t0rn collator two secret'\""
  # fx: ssh alibaba@000.00.00.00 'bash -s' < ./run-rococo-collators.sh v0.0.0-roco "'secret one'" "'secret two'"
  exit 1
fi

tag=$1
t0rn_collator_one_secret="$2"
t0rn_collator_two_secret="$3"
collator_binary=/home/atlas/rococo/circuit-collator
sleep_binary=$(which sleep)
artifacts_dir=/home/atlas/rococo/specs
t0rn_collator_one_home=/home/atlas/rococo/t0rn-collator-one
t0rn_collator_two_home=/home/atlas/rococo/t0rn-collator-two
systemd_user_service_dir=/home/atlas/.config/systemd/user

# prep permanent locations
mkdir -p \
  /home/atlas/rococo \
  $t0rn_collator_one_home \
  $t0rn_collator_two_home \
  $artifacts_dir \
  $systemd_user_service_dir

# pull a prebuilt circuit collator binary
curl -sSfL \
  https://github.com/t3rn/t3rn/releases/download/$tag/t0rn-collator-$tag-x86_64-unknown-linux-gnu.gz \
| \
gunzip \
> $collator_binary
chmod +x $collator_binary

# copy semi static chain specs
curl -sSfL \
  -o $artifacts_dir/rococo.raw.json \
  https://raw.githubusercontent.com/t3rn/t3rn/$tag/specs/rococo.raw.json

curl -sSfL \
  -o $artifacts_dir/t0rn.genesis \
  https://raw.githubusercontent.com/t3rn/t3rn/$tag/specs/t0rn.genesis

curl -sSfL \
  -o $artifacts_dir/t0rn.json \
  https://raw.githubusercontent.com/t3rn/t3rn/$tag/specs/t0rn.json

curl -sSfL \
  -o $artifacts_dir/t0rn.raw.json \
  https://raw.githubusercontent.com/t3rn/t3rn/$tag/specs/t0rn.raw.json

curl -sSfL \
  -o $artifacts_dir/t0rn.wasm \
  https://raw.githubusercontent.com/t3rn/t3rn/$tag/specs/t0rn.wasm

# boot node(s) - FIXME: gt 1; space/comma-separated list not valid
rococo_boot_nodes="$(jq -r .bootNodes[0] $artifacts_dir/rococo.raw.json)"

# define the t0rn-collator-one systemd service
echo "
[Unit]
Description=t0rn-collator-one
After=network.target
StartLimitIntervalSec=1d
StartLimitBurst=5

[Service]
Type=simple
Restart=on-failure
RestartSec=10s
ExecStartPre=$collator_binary \
  purge-chain \
  -y \
  --base-path $t0rn_collator_one_home \
  --chain $artifacts_dir/t0rn.raw.json
ExecStart=$collator_binary \
  --collator \
  --name t0rn-collator-one \
  --base-path $t0rn_collator_one_home \
  --chain $artifacts_dir/t0rn.raw.json \
  --discover-local \
  --port 33333 \
  --rpc-port 8833 \
  --ws-port 9933 \
  --unsafe-ws-external \
  --rpc-cors all \
  --execution Wasm \
  --pruning=archive \
  --prometheus-port 7001 \
  --telemetry-url 'wss://telemetry.polkadot.io/submit 1' \
  -- \
  --chain $artifacts_dir/rococo.raw.json \
  --bootnodes \"$rococo_boot_nodes\" \
  --port 10001 \
  --rpc-port 8001 \
  --ws-port 9001 \
  --execution Wasm \
  --no-prometheus
ExecStartPost=$sleep_binary 33s
ExecStartPost=$collator_binary \
  key \
  insert \
  --base-path $t0rn_collator_one_home \
  --chain $artifacts_dir/t0rn.raw.json \
  --scheme Sr25519 \
  --suri \"$t0rn_collator_one_secret\" \
  --key-type aura

[Install]
WantedBy=multi-user.target
" > $systemd_user_service_dir/t0rn-collator-one.service

# define the t0rn-collator-two systemd service
echo "
[Unit]
Description=t0rn-collator-two
After=network.target
StartLimitIntervalSec=1d
StartLimitBurst=5

[Service]
Type=simple
Restart=on-failure
RestartSec=10s
ExecStartPre=$collator_binary \
  purge-chain \
  -y \
  --base-path $t0rn_collator_two_home \
  --chain $artifacts_dir/t0rn.raw.json
ExecStart=$collator_binary \
  --collator \
  --name t0rn-collator-two \
  --base-path $t0rn_collator_two_home \
  --chain $artifacts_dir/t0rn.raw.json \
  --discover-local \
  --port 33332 \
  --rpc-port 8832 \
  --ws-port 9932 \
  --unsafe-ws-external \
  --rpc-cors all \
  --execution Wasm \
  --pruning=archive \
  --prometheus-port 7002 \
  --telemetry-url 'wss://telemetry.polkadot.io/submit 1' \
  -- \
  --chain $artifacts_dir/rococo.raw.json \
  --bootnodes \"$rococo_boot_nodes\" \
  --port 10002 \
  --rpc-port 8002 \
  --ws-port 9002 \
  --execution Wasm \
  --no-prometheus
ExecStartPost=$sleep_binary 33s
ExecStartPost=$collator_binary \
  key \
  insert \
  --base-path $t0rn_collator_two_home \
  --chain $artifacts_dir/t0rn.raw.json \
  --scheme Sr25519 \
  --suri \"$t0rn_collator_two_secret\" \
  --key-type aura

[Install]
WantedBy=multi-user.target
" > $systemd_user_service_dir/t0rn-collator-two.service

# start the collators...
systemctl --user enable t0rn-collator-one.service
systemctl --user enable t0rn-collator-two.service
systemctl --user start t0rn-collator-one.service
systemctl --user start t0rn-collator-two.service

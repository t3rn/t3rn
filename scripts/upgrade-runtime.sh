#!/bin/bash

set -xEe

if [[ -z "$1" || -z $2 || -z $3 ]]; then
  echo "usage: $0 'sudo secret' \$provider \$tag"
  # fx: ./upgrade-runtime.sh 'sudo secret' wss://dev.net.t3rn.io v3.3.3
  exit 1
fi

sudo_secret="$1"
provider=$2
tag=$3

git checkout $tag

root_dir=$(git rev-parse --show-toplevel)

if ! cargo install --list | grep -q 'srtool-cli v0.8.0'; then
  cargo install \
    --git https://github.com/chevdor/srtool-cli \
    --tag v0.8.0
fi

echo "compiling runtime wasm..."

report="$( \
  srtool build \
    --profile release \
    --runtime-dir runtime/parachain \
    --package circuit-parachain-runtime \
    --json \
    $root_dir \
)"

# left trimming nonjson
report="{${report#*\{}"
wasm=$(jq -r .runtimes.compact.wasm <<<"$report")
hash=$(jq -r .runtimes.compact.blake2_256 <<<"$report")
hex_wasm_runtime=$(mktemp)
# xxd from vim
printf "0x$(cat $wasm | tr -d '\n' | xxd -p | tr -d '\n')" > $hex_wasm
authorize_upgrade_call_data="0x0102${hash#0x}"

# fetch authoring_version, spec_version, impl_version, and transaction_version from live chain
# grep authoring_version, spec_version, impl_version, and transaction_version from tagged files
# mk sure authoring_version, spec_version, impl_version, and transaction_version incremented

# if possible check whether any storage layout has changed
# if yes assert migrations are included

# assert that benchmarks are up2date
# ./target/release/node-template benchmark \
#     --chain dev \               # Configurable Chain Spec
#     --execution wasm \          # Always test with Wasm
#     --wasm-execution compiled \ # Always used `wasm-time`
#     --pallet pallet_example \   # Select the pallet
#     --extrinsic '*' \          # Select the benchmark case name, using '*' for all
#     --steps 20 \                # Number of steps across component ranges
#     --repeat 10 \               # Number of times we repeat a benchmark
#     --json-file=raw.json \      # Optionally output json benchmark data to a file
#     --output ./                 # Output results into a Rust file

# https://docs.substrate.io/rustdocs/latest/sp_version/struct.RuntimeVersion.html
read -n 1 -p "e2e-tested on rococo-local?
runtime benchmarked?
storage migrated?
authoring_version incremented?
spec_version incremented?
impl_version incremented?
transaction_version incremented?
(y/n) " answer

echo

if [[ "${answer,,}" != "y" ]]; then exit 1; fi

echo "authorizing runtime upgrade..."

npx --yes @polkadot/api-cli@beta \
  --ws $provider \
  --sudo \
  --seed "$sudo_secret" \
  tx.parachainSystem.authorizeUpgrade \
  $authorize_upgrade_call_data

echo "enacting runtime upgrade..."

npx @polkadot/api-cli@beta \
  --ws $provider \
  --seed "$sudo_secret" \
  --params $hex_wasm_runtime \
  tx.parachainSystem.enactAuthorizedUpgrade
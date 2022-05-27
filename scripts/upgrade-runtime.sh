#!/bin/bash

set -x

if [[ -z $1 || -z "$2" || -z $3 || -z $4 ]]; then
  echo "usage: $0 \$relay_chain 'collator sudo secret' \$provider \$tag [--dryrun]"
  # fx: ./upgrade-runtime.sh rococo 'collator sudo secret' wss://dev.net.t3rn.io v3.3.3
  exit 1
fi

relay_chain=$1
sudo_secret="$2"
provider=$3
tag=$4
root_dir=$(git rev-parse --show-toplevel)
dryrun=$(echo "$@" | grep -o dry)

if [[ "$relay_chain" -ne rococo ]]; then
  # 4 pdot parachain runtime upgrades we at least need the relaychain spec
  echo -e "polkadot parachain runtime upgrades are not supported yet" >&2
  exit 1
fi

if ! git tag --list | grep -Fq $tag; then
  echo -e "$tag is not a git tag\ntag and push the runtime for the upgrade" >&2
  exit 1
fi

if ! cargo install --list | grep -Fq 'srtool-cli v0.8.0'; then
  echo "installing srtool-cli..."
  cargo install \
    --git https://github.com/chevdor/srtool-cli \
    --tag v0.8.0
fi

echo "checking out $tag..."

git checkout $tag

echo "making sure runtime version got updated..."

set -Ee

# fetch authoring_version, spec_version, impl_version, and transaction_version from live chain
runtime_version="$( \
  npx --yes @polkadot/api-cli@beta \
    --ws $provider \
    consts.system.version \
)"
old_spec_version=$(jq -r .version.specVersion <<<"$runtime_version")
old_impl_version=$(jq -r .version.implVersion <<<"$runtime_version")
old_tx_version=$(jq -r .version.transactionVersion <<<"$runtime_version")
old_author_version=$(jq -r .version.authoringVersion <<<"$runtime_version")

# grep authoring_version, spec_version, impl_version, and transaction_version from tagged files
new_spec_version=$(grep -Pom1 'spec_version: *\K[0-9]+' ../runtime/parachain/src/lib.rs)
new_impl_version=$(grep -Pom1 'impl_version: *\K[0-9]+' ../runtime/parachain/src/lib.rs)
new_tx_version=$(grep -Pom1 'transaction_version: *\K[0-9]+' ../runtime/parachain/src/lib.rs)
new_author_version=$(grep -Pom1 'authoring_version: *\K[0-9]+' ../runtime/parachain/src/lib.rs)

# mk sure authoring_version, spec_version, impl_version, and transaction_version incremented
if [[ "$new_spec_version" -ne "$((old_spec_version + 1))" ]]; then
  echo "runtime spec version not incremented" >&2
  exit 1
fi

if [[ "$new_impl_version" -ne "$((old_impl_version + 1))" ]]; then
  echo "runtime impl version not incremented" >&2
  exit 1
fi

if [[ "$new_tx_version" -ne "$((old_tx_version + 1))" ]]; then
  echo "runtime transaction version not incremented" >&2
  exit 1
fi

if [[ "$new_author_version" -ne "$((old_author_version + 1))" ]]; then
  echo "runtime authoring version not incremented" >&2
  exit 1
fi

echo "compiling runtime wasm..."

report="$( \
  srtool build \
    --profile release \
    --runtime-dir runtime/parachain \
    --package circuit-parachain-runtime \
    --default-features runtime-benchmarks \
    --json \
    $root_dir \
)"

report="{${report#*\{}" # left trimming nonjson
wasm="$root_dir/$(jq -r .runtimes.compact.wasm <<<"$report")"
hash=$(jq -r .runtimes.compact.blake2_256 <<<"$report")
hex_wasm_runtime=$(mktemp)         # xxd from vim
printf "0x$(cat $wasm | tr -d '\n' | xxd -p | tr -d '\n')" > $hex_wasm_runtime
authorize_upgrade_call_data="0x0102${hash#0x}"

read -n 1 -p "e2e-tested on rococo-local?
runtime upgrade tested on rococo-local?
runtime benchmarked?
storage migrated?
(y/n) " answer

echo

if [[ "${answer,,}" -ne "y" ]]; then exit 1; fi

echo "authorizing runtime upgrade... $dryrun"

if [[ -n $dryrun ]]; then
  echo "
  npx --yes @polkadot/api-cli@beta \
    --ws $provider \
    --sudo \
    --seed "$sudo_secret" \
    tx.parachainSystem.authorizeUpgrade \
    $authorize_upgrade_call_data
  "
else
    npx --yes @polkadot/api-cli@beta \
    --ws $provider \
    --sudo \
    --seed "$sudo_secret" \
    tx.parachainSystem.authorizeUpgrade \
    $authorize_upgrade_call_data
fi

echo "enacting runtime upgrade... $dryrun" # TODO: wrap with scheduler

if [[ -n $dryrun ]]; then
  echo "
  npx @polkadot/api-cli@beta \
    --ws $provider \
    --seed \"$sudo_secret\" \
    --params $hex_wasm_runtime \
    tx.parachainSystem.enactAuthorizedUpgrade
  "
else
  npx @polkadot/api-cli@beta \
    --ws $provider \
    --seed "$sudo_secret" \
    --params $hex_wasm_runtime \
    tx.parachainSystem.enactAuthorizedUpgrade
fi
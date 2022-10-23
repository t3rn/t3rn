#!/bin/bash

set -Eex

case "$(uname -s)" in
  Linux*)   machine=linux;;
  Darwin*)  machine=macos;;
  *)        exit 1;;
esac

provider=${ZOMBIENET_PROVIDER:-native}
version=${ZOMBIENET_VERSION:-v1.2.59}
runtime=${RUNTIME:-t3rn-parachain}
pdot_branch=${PDOT_BRANCH:-release-v0.9.27}
root_dir=$(git rev-parse --show-toplevel)

mkdir -p /tmp/zombienet/bin

if [[ ! -x /usr/local/bin/zombienet ]]; then
  curl -fL# -o /usr/local/bin/zombienet https://github.com/paritytech/zombienet/releases/download/$version/zombienet-$machine
  chmod +x /usr/local/bin/zombienet
fi

if [[ ! -x /tmp/zombienet/bin/polkadot ]]; then
  tmp_dir=$(mktemp -d)
  git clone --branch $pdot_branch --depth 1 https://github.com/paritytech/polkadot $tmp_dir
  cargo build --manifest-path $tmp_dir/Cargo.toml --features fast-runtime --release --locked
  mv -f $tmp_dir/target/release/polkadot /tmp/zombienet/bin/polkadot
fi

cargo build --manifest-path $root_dir/node/$runtime/Cargo.toml --release --locked
cp -f $root_dir/target/release/circuit-collator /tmp/zombienet/bin/circuit-collator

PATH=/tmp/zombienet/bin:$PATH /usr/local/bin/zombienet --provider=$provider spawn $root_dir/zombienet.toml
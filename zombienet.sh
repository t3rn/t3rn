#!/bin/bash

set -Eex

case "$(uname -s)" in
  Linux*)   machine=linux;;
  Darwin*)  machine=macos;;
  *)        exit 1;;
esac

provider=${ZOMBIENET_PROVIDER:-native}
version=${ZOMBIENET_VERSION:-v1.2.59}
runtime=${RUNTIME:-t3rn}
pdot_branch=${PDOT_BRANCH:-release-v0.9.27}
root_dir=$(git rev-parse --show-toplevel)
[ ! -O /usr/local/bin ] && sudo_maybe=sudo

bin_dir=$root_dir/bin
mkdir -p $bin_dir

if [[ ! -x $bin_dir/zombienet ]]; then
  curl -fL# -o $bin_dir/zombienet https://github.com/paritytech/zombienet/releases/download/$version/zombienet-$machine
  echo "#### Need sudo access for zombienet executable ####"
  $sudo_maybe chmod +x $bin_dir/zombienet
fi

if [[ ! -x $root_dir/bin/polkadot ]]; then
  tmp_dir=$(mktemp -d)
  git clone --branch $pdot_branch --depth 1 https://github.com/paritytech/polkadot $tmp_dir
  cargo build --manifest-path $tmp_dir/Cargo.toml --features fast-runtime --release --locked
  mv -f $tmp_dir/target/release/polkadot $root_dir/bin/polkadot
fi

cargo build --manifest-path $root_dir/node/$runtime-parachain/Cargo.toml --release --locked
cp -f $root_dir/target/release/$runtime-collator $root_dir/bin/circuit-collator

PATH=$bin_dir:$PATH $bin_dir/zombienet --provider=$provider spawn $root_dir/zombienet.toml

# TODO: expose functions here
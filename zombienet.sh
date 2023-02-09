#!/bin/bash

set -Eex

case "$(uname -s)" in
  Linux*)   machine=linux;;
  Darwin*)  machine=macos;;
  *)        exit 1;;
esac

provider=${ZOMBIENET_PROVIDER:-native}
version=${ZOMBIENET_VERSION:-v1.2.59}
name=${NAME:-t3rn}
runtime=${RUNTIME:-t3rn-parachain}
pdot_branch=${PDOT_BRANCH:-release-v0.9.37}
root_dir=$(git rev-parse --show-toplevel)
[ ! -O /usr/local/bin ] && sudo_maybe=sudo

mkdir -p $root_dir/bin

if [[ ! -x /usr/local/bin/zombienet ]]; then
  $sudo_maybe curl -fL# -o /usr/local/bin/zombienet https://github.com/paritytech/zombienet/releases/download/$version/zombienet-$machine
  $sudo_maybe chmod +x /usr/local/bin/zombienet
fi

if [[ ! -x $root_dir/bin/polkadot ]]; then
  tmp_dir=/tmp/pdot-$pdot_branch
  if [[ ! -d $tmp_dir ]]; then
    mkdir -p $tmp_dir
    git clone --branch $pdot_branch --depth 1 https://github.com/paritytech/polkadot $tmp_dir
    cargo build --manifest-path $tmp_dir/Cargo.toml --features fast-runtime --release --locked
  fi
  cp $tmp_dir/target/release/polkadot $root_dir/bin/polkadot
fi

cargo build --manifest-path $root_dir/node/$runtime/Cargo.toml --release --locked
cp -f $root_dir/target/release/$name-collator $root_dir/bin/$name-collator

PATH=$root_dir/bin:$PATH /usr/local/bin/zombienet --provider=$provider spawn $root_dir/zombienet.toml
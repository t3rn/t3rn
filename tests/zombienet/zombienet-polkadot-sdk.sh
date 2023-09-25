#!/usr/bin/env bash

set -e

root_dir=$(git rev-parse --show-toplevel)
bin_dir="$root_dir/bin"
working_dir="$(pwd)"

provider=native
zombienet_version=v1.3.64
pdot_branch=release-v1.0.0
pdot_sdk_cumulus_branch=mergebase-cumulus-v1.0.0
pdot_sdk_polkadot_branch=mergebase-polkadot-v1.0.0
polkadot_tmp_dir=/tmp/polkadot
asset_hub_tmp_dir=/tmp/asset_hub


arch=$(uname -s 2>/dev/null || echo not)
if [[ $arch == "Darwin" ]]; then
    machine=macos
elif [[ $arch == "Linux" ]]; then
    machine=linux-x64
fi

PATH=$PATH:$bin_dir

clean() {
    echo "Cleaning bin dir"
    rm -rf "$bin_dir"/*
}

make_bin_dir() {
    echo "Making bin dir"
    mkdir -p "$bin_dir"
}

fetch_zombienet() {
    # Don't fetch zombienet if it's already present in the system
    if which zombienet-$zombienet_version >/dev/null; then
        cp $(which zombienet-$zombienet_version) "$bin_dir/zombienet"
        echo "✅ zombienet-$zombienet_version"
        return
    fi

    if [ ! -f "$bin_dir/zombienet" ]; then
        echo "::group::Install zombienet"
        echo "Fetching zombienet..."
        curl -fL -o "$bin_dir/zombienet" "https://github.com/paritytech/zombienet/releases/download/$zombienet_version/zombienet-$machine"

        echo "Making zombienet executable"
        chmod +x "$bin_dir/zombienet"
        echo "::endgroup::"
        echo "✅ zombienet-$zombienet_version"
    else
        echo "✅ zombienet-$zombienet_version"
    fi
}

build_polkadot() {
    if [ -f "$bin_dir/polkadot" ]; then
        echo "✅ polkadot-$pdot_branch"
        return
    fi

    if [ ! -f "$polkadot_tmp_dir/$pdot_branch/polkadot/target/release/polkadot" ]; then
        echo "::group::Install polkadot."
        echo "Cloning polkadot into $polkadot_tmp_dir"
        mkdir -p "$polkadot_tmp_dir"
        git clone --branch "$pdot_sdk_polkadot_branch" https://github.com/paritytech/polkadot-sdk "$polkadot_tmp_dir/$pdot_branch" || true
        echo "Building polkadot..."
        cargo build --manifest-path "$polkadot_tmp_dir/$pdot_branch/polkadot/Cargo.toml" --features fast-runtime --release --locked
        cp "$polkadot_tmp_dir/$pdot_branch/polkadot/target/release/polkadot" "$bin_dir/polkadot"
        echo "::endgroup::"
        echo "✅ polkadot-$pdot_branch"
    else
        cp "$polkadot_tmp_dir/$pdot_branch/polkadot/target/release/polkadot" "$bin_dir/polkadot"
        echo "✅ polkadot-$pdot_branch"
    fi
}

build_asset_hub() {
    if [ -f "$bin_dir/asset-hub" ]; then
        echo "✅ asset-hub-$pdot_branch"
        return
    fi

    if [ ! -f "$asset_hub_tmp_dir/$pdot_branch/cumulus/target/release/polkadot-parachain" ]; then
        echo "::group::Install AssetHub."
        echo "Cloning AssetHub into $asset_hub_tmp_dir"
        mkdir -p "$asset_hub_tmp_dir"
        git clone --branch "$pdot_sdk_cumulus_branch" https://github.com/paritytech/polkadot-sdk "$asset_hub_tmp_dir/$pdot_branch" || true
        echo "Building AssetHub..."
        cargo build --manifest-path "$asset_hub_tmp_dir/$pdot_branch/cumulus/Cargo.toml" --release --locked --bin polkadot-parachain
        cp "$asset_hub_tmp_dir/$pdot_branch/cumulus/target/release/polkadot-parachain" "$bin_dir/asset-hub"
        echo "::endgroup::"
        echo "✅ asset-hub-$pdot_branch"
    else
        cp "$asset_hub_tmp_dir/$pdot_branch/cumulus/target/release/polkadot-parachain" "$bin_dir/asset-hub"
        echo "✅ asset-hub-$pdot_branch"
    fi
}

NODE_ARG=t3rn

build_collator() {
    if [ ! -f "$bin_dir/$NODE_ARG-collator" ]; then
        echo "::group::Building $NODE_ARG..."
        time cargo build --manifest-path "$root_dir/node/$NODE_ARG-parachain/Cargo.toml" --release --locked
        echo "::endgroup::"

        echo "Copying $NODE_ARG to bin dir"
        cp "$root_dir/target/release/$NODE_ARG-collator" "$bin_dir/"
    else
        echo "✅ $NODE_ARG already built"
    fi
}

force_build_collator() {
    echo "::group::Rebuilding $NODE_ARG..."
    time cargo build --manifest-path "$root_dir/node/$NODE_ARG-parachain/Cargo.toml" --release
    echo "::endgroup::"
    
    cp "$root_dir/target/release/$NODE_ARG-collator" "$bin_dir/"
}

setup() {
    make_bin_dir
    fetch_zombienet
    build_polkadot
    build_asset_hub

    NODE_ARG=t0rn
    build_collator
}

smoke() {
    echo "Running smoke tests.."
    # TODO[Optimisation]: loop through directory and test all
    # TODO[Optimisation, NotImplemented]: when zombienet can run on a pre-existing network, run it
    echo "::group::Zombienet tests..."
    time zombienet --provider="$provider" test $working_dir/smoke/0001-is_up_and_registered.feature
    echo "::endgroup::"
}

upgrade() {
    if [[ $# -ne 2 ]]; then
        echo "Expecting exactly 2 arguments"
        echo $@
        echo "Usage: ./zombienet.sh upgrade <t3rn/t0rn>"
        return 1
    fi
    
    parachain=$2

    [[ "$machine" = "macos" ]] && echo "We release binaries on Github only for x86" && exit 1

    echo "Testing real upgrade for parachain: ${parachain}"
    
    # Fetch latest release binary from Github
    git fetch --all --tags -f || true > /dev/null
    
    $working_dir/download.sh "$parachain"
    
    # Run collator and upgrade with built WASM binary
    zombienet --provider="$provider" test $working_dir/smoke/9999-runtime_upgrade.feature

    echo "Upgrade tests succeed!"
}


upgrade_local() {
    if [[ $# -ne 2 ]]; then
        echo "Expecting exactly 2 arguments"
        echo $@
        echo "Usage: ./zombienet.sh upgrade <t3rn/t0rn>"
        return 1
    fi
    
    parachain=$2

    [[ "$machine" = "macos" ]] && echo "We release binaries on Github only for x86" && exit 1

    echo "Testing real upgrade for parachain: ${parachain}"
    
    # Fetch latest release binary from Github
    git fetch --all --tags -f || true > /dev/null
    
    $working_dir/download_local.sh "$parachain"
    
    # Run collator and upgrade with built WASM binary
    zombienet --provider="$provider" test $working_dir/smoke/9999-runtime_upgrade.feature

    echo "Upgrade tests succeed!"
}

confirm_sfx() {
    docker-compose up -d grandpa-ranger
    echo "Spawning zombienet in the background..."
    zombienet --provider="$provider" test $working_dir/sfx/0000-confirm_sfx.zndsl
    docker-compose down
}

spawn() {
    echo "Spawning zombienet using provider: $provider..."
    zombienet --provider="$provider" spawn ./zombienet.toml
}

case "$1" in
  "setup")
      setup
      ;;
  "smoke")
      setup
      smoke
      ;;
  "sfx")
      make_bin_dir
      fetch_zombienet
      build_polkadot
      confirm_sfx
      ;;
  "upgrade")
      make_bin_dir
      fetch_zombienet
      build_polkadot
      upgrade $@
      ;;
  "upgrade_local")
      setup
      upgrade_local $@
      ;;
  "spawn")
      setup
      spawn
      ;;
  "force_build_collator")
      force_build_collator
      ;;
  *)
      echo "Enter an appropriate command"
      exit 1
      ;;
esac
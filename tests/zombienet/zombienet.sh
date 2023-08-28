#!/usr/bin/env bash

set -e

root_dir=$(git rev-parse --show-toplevel)
bin_dir="$root_dir/bin"
working_dir="$(pwd)"

provider=native
zombienet_version=v1.3.64
pdot_branch=release-v1.0.0
polkadot_tmp_dir=/tmp/polkadot


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

    if [ ! -f "$polkadot_tmp_dir/$pdot_branch/target/release/polkadot" ]; then
        echo "::group::Install polkadot."
        echo "Cloning polkadot into $polkadot_tmp_dir"
        mkdir -p "$polkadot_tmp_dir"
        git clone --branch "$pdot_branch" --depth 1 https://github.com/paritytech/polkadot "$polkadot_tmp_dir/$pdot_branch" || true
        echo "Building polkadot..."
        cargo build --manifest-path "$polkadot_tmp_dir/$pdot_branch/Cargo.toml" --features fast-runtime --release --locked
        cp "$polkadot_tmp_dir/$pdot_branch/target/release/polkadot" "$bin_dir/polkadot"
        echo "::endgroup::"
        echo "✅ polkadot-$pdot_branch"
    else
        cp "$polkadot_tmp_dir/$pdot_branch/target/release/polkadot" "$bin_dir/polkadot"
        echo "✅ polkadot-$pdot_branch"
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
    echo "Spawning zombienet in the background..."
    zombienet --provider="$provider" test $working_dir/smoke/1000-confirm_sfx.feature

    spawn_executor_and_run_tests &

    echo "Executor started ✅"
    # Wait for the tests to complete or timeout
    end_time=$(($(date +%s) + 300)) # Set the end time for the loop to 300 seconds from now
    while [ $(date +%s) -lt $end_time ] && kill -0 $executor_pid >/dev/null 2>&1; do
        sleep 1
    done
    if [ $(date +%s) -ge $end_time ]; then
        echo "zombienet.sh: tests timed out ❌"
        cleanup # Call the cleanup function to send SIGINT signals to the processes
        exit 1
    elif [ $? -eq 0 ]; then
        echo "zombienet.sh: tests passed ✅"
        cleanup # Call the cleanup function to send SIGINT signals to the processes
        exit 0
    else
        echo "zombienet.sh: tests failed ❌"
        cleanup # Call the cleanup function to send SIGINT signals to the processes
        exit 1
    fi
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
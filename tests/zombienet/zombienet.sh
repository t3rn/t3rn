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
        echo "Fetching zombienet..."
        curl -fL -o "$bin_dir/zombienet" "https://github.com/paritytech/zombienet/releases/download/$zombienet_version/zombienet-$machine"

        echo "Making zombienet executable"
        chmod +x "$bin_dir/zombienet"
        echo "✅ zombienet-$zombienet_version"
    else
        echo "✅ zombienet-$zombienet_version"
    fi
}

build_polkadot() {
    if [ ! -f "$polkadot_tmp_dir/$pdot_branch/target/release/polkadot" ]; then
        echo "Cloning polkadot into $polkadot_tmp_dir"
        mkdir -p "$polkadot_tmp_dir"
        git clone --branch "$pdot_branch" --depth 1 https://github.com/paritytech/polkadot "$polkadot_tmp_dir/$pdot_branch"
        echo "Building polkadot..."
        cargo build --manifest-path "$polkadot_tmp_dir/$pdot_branch/Cargo.toml" --features fast-runtime --release --locked
        cp "$polkadot_tmp_dir/$pdot_branch/target/release/polkadot" "$bin_dir/polkadot"
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

    NODE_ARG=t1rn
    build_collator

    NODE_ARG=t3rn
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

spawn_and_confirm_xtx() {
    # Function to handle signals sent to the script
    cleanup() {
        # Send SIGINT signal to the zombienet process
        kill -s SIGINT $zombienet_pid
        # Send SIGINT signal to the executor process
        kill -s SIGINT $executor_pid
    }
    echo "Spawning zombienet in the background..."
    nohup zombienet --provider=native spawn ./zombienet.toml > /dev/null 2>&1 &
    zombienet_pid=$! # Save the PID of the zombienet process
    echo "Zombienet PID: $zombienet_pid"
    trap 'cleanup' INT TERM # Set up signal traps
    # Wait for the zombienet process to start
    for i in $(seq 1 10); do
        if kill -0 $zombienet_pid >/dev/null 2>&1; then
            break # Exit the loop if the zombienet process has started
        else
            sleep 1 # Wait for 1 second
        fi
    done
    if ! kill -0 $zombienet_pid >/dev/null 2>&1; then
        # The zombienet process failed to start
        echo "zombienet.sh: zombienet failed to start ❌"
        exit 1
    fi
    echo "Zombienet started ✅"
    # Call the function that spawns the executor and runs the tests
    spawn_executor_and_run_tests &
    executor_pid=$! # Save the PID of the executor process
    echo "Executor PID: $executor_pid"
    # Wait for the executor process to start
    for i in $(seq 1 10); do
        if pgrep -P $executor_pid >/dev/null 2>&1; then
            break # Exit the loop if the executor process has started
        else
            sleep 1 # Wait for 1 second
        fi
    done
    if ! pgrep -P $executor_pid >/dev/null 2>&1; then
        # The executor process failed to start
        echo "zombienet.sh: start_executor failed ❌"
        cleanup # Call the cleanup function to send SIGINT signals to the processes
        exit 1
    fi
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

spawn_executor_and_run_tests() {
    echo "Installing client/packages with make all ⏳"
    make all -C ../../client/packages
    if [ $? -ne 0 ]; then
        echo "zombienet.sh: make all failed ❌"
        cleanup # Call the cleanup function to send SIGINT signals to the processes
        exit 1
    fi
    echo "client/packages:: make all done - client packages assumed to be installed ✅"
    echo "Starting executor ⏳"
    WS_CIRCUIT_ENDPOINT=ws://127.0.0.1:9940 make start_executor -C ../../client/packages &
    executor_pid=$! # Save the PID of the executor process
    # Wait for the executor process to start
    for i in $(seq 1 10); do
        if pgrep -P $executor_pid >/dev/null 2>&1; then
            break # Exit the loop if the executor process has started
        else
            sleep 1 # Wait for 1 second
        fi
    done
    if ! pgrep -P $executor_pid >/dev/null 2>&1; then
        # The executor process failed to start
        echo "zombienet.sh: start_executor failed ❌"
        cleanup # Call the cleanup function to send SIGINT signals to the processes
        exit 1
    fi
    echo "Executor started ✅"
    echo "Running make::test_confirm_xtx sending single SFX on top of Rococo header ranges ⏳"
    WS_CIRCUIT_ENDPOINT=ws://127.0.0.1:9940 make test_confirm_xtx -C ../../client/packages
    if [ $? -ne 0 ]; then
        # The test failed
        echo "zombienet.sh: test_confirm_xtx failed ❌"
        cleanup # Call the cleanup function to send SIGINT signals to the processes
        exit 1
    fi
    echo "Test confirm SFX passed ✅"
    # Call the cleanup function to send SIGINT signals to the processes
    cleanup
}

upgrade() {
    if [[ $# -ne 2 ]]; then
        echo "Expecting exactly 2 arguments"
        echo $@
        echo "Usage: ./zombienet.sh upgrade <t3rn/t1rn/t0rn>"
        return 1
    fi
    
    parachain=$2

    [[ "$machine" = "macos" ]] && echo "You're on macos, this is for CI :< - it uses previously built binaries from CI only" && exit 1

    # run tests
    echo "Testing real upgrade for parachain: ${parachain}"
    
    # get last release binary from github
    git fetch --all --tags -f || true
    
    echo $working_dir
    echo $parachain
    source $working_dir/download.sh "$parachain"
    
    # deploy with test (ensuring old binary, new blob)
    zombienet --provider="$provider" test $working_dir/smoke/9999-runtime_upgrade.feature

    echo "Upgrade tests succeed!"
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
      force_build_collator
      smoke
      ;;
  "spawn_and_confirm_xtx")
      setup
      spawn_and_confirm_xtx
      ;;
  "upgrade")
      make_bin_dir
      fetch_zombienet
      build_polkadot

      upgrade $@
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
      ;;
esac
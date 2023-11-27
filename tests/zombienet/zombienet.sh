#!/usr/bin/env bash

set -e

root_dir=$(git rev-parse --show-toplevel)
bin_dir="$root_dir/bin"
working_dir="$(pwd)"

provider=native
zombienet_version=v1.3.64
pdot_branch=release-v1.0.0
polkadot_tmp_dir=/tmp/polkadot
asset_hub_tmp_dir=/tmp/asset_hub

NETWORK=${2:-t0rn}
echo "🦄 Running zombienet for network ${NETWORK}"

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

build_asset_hub() {
    if [ -f "$bin_dir/asset-hub" ]; then
        echo "✅ asset-hub-$pdot_branch"
        return
    fi
    
    if [ ! -f "$asset_hub_tmp_dir/$pdot_branch/target/release/polkadot-parachain" ]; then
        echo "::group::Install AssetHub."
        echo "Cloning AssetHub into $asset_hub_tmp_dir"
        mkdir -p "$asset_hub_tmp_dir"
        git clone --branch "$pdot_branch" --depth 1 https://github.com/paritytech/cumulus "$asset_hub_tmp_dir/$pdot_branch" || true
        echo "Building AssetHub..."
        cargo build --manifest-path "$asset_hub_tmp_dir/$pdot_branch/Cargo.toml" --release --locked --bin polkadot-parachain
        cp "$asset_hub_tmp_dir/$pdot_branch/target/release/polkadot-parachain" "$bin_dir/asset-hub"
        echo "::endgroup::"
        echo "✅ asset-hub-$pdot_branch"
    else
        cp "$asset_hub_tmp_dir/$pdot_branch/target/release/polkadot-parachain" "$bin_dir/asset-hub"
        echo "✅ asset-hub-$pdot_branch"
    fi
}


build_collator() {
    if [ ! -f "$bin_dir/$NETWORK-collator" ]; then
        echo "::group::Building $NETWORK..."
        time cargo build --manifest-path "$root_dir/node/$NETWORK-parachain/Cargo.toml" --release --locked
        echo "::endgroup::"
        cp "$root_dir/target/release/$NETWORK-collator" "$bin_dir/${NETWORK}-collator"
    fi
    echo "✅ $NETWORK built"
    cp "$root_dir/target/release/$NETWORK-collator" "$bin_dir/collator"
    
    echo Current version of collator:
    "$bin_dir/collator" -V
}

force_build_collator() {
    echo "::group::Rebuilding $NETWORK..."
    time cargo build --manifest-path "$root_dir/node/$NETWORK-parachain/Cargo.toml" --release
    echo "::endgroup::"
    
    cp "$root_dir/target/release/$NETWORK-collator" "$bin_dir/"
}

setup() {
    make_bin_dir
    fetch_zombienet
    build_polkadot
    
    build_collator
}

setup_xcm() {
    make_bin_dir
    fetch_zombienet
    build_polkadot
    build_asset_hub
    build_collator
}

setup_alles() {
    make_bin_dir
    fetch_zombienet
    build_polkadot
    NETWORK=t0rn
    build_collator
    NETWORK=t1rn
    build_collator
    NETWORK=t3rn
    build_collator
    NETWORK=t7rn
    build_collator
}

smoke() {
    echo "Running smoke tests.."
    # TODO[Optimisation]: loop through directory and test all
    # TODO[Optimisation, NotImplemented]: when zombienet can run on a pre-existing network, run it
    echo "::group::Zombienet tests..."
    if [ "$NETWORK" = "t0rn" ]; then
        time zombienet --provider="$provider" test $working_dir/smoke/0001-is_up.zndsl
    else
        time zombienet --provider="$provider" test $working_dir/smoke/0001-is_up_t1rn.zndsl
    fi
    echo "::endgroup::"
}

runtime_upgrade() {
    if [[ $# -lt 2 ]]; then
        echo "Expecting 2 or more arguments"
        echo $@
        echo "Usage: ./zombienet.sh upgrade <t3rn/t1rn/t0rn> [local/github]"
        return 1
    fi
    
    parachain=$2
    
    [[ "$machine" = "macos" ]] && echo "We release binaries on Github only for x86" && exit 1
    
    echo "🧹 Cleaning bin dir"
    rm -f ${bin_dir}/*collator*
    rm -f ${bin_dir}/*.wasm
    
    echo "⏳ Testing real upgrade for parachain: ${parachain}"
    echo "::group::Building..."
    if [[ "$3" == "local" ]]; then
        echo "Using local WASM binaries"
        $working_dir/download_local.sh "$parachain"
        
    else
        echo "Using Github WASM binaries from latest release"
        
        # Fetch latest release binary from Github
        $working_dir/download.sh "$parachain"
    fi
    subwasm info ${bin_dir}/parachain_runtime.compact.compressed.wasm
    echo "::endgroup::"
    
    # Run collator and upgrade with built WASM binary
    echo "::group::Zombienet tests..."
    if [ "$NETWORK" = "t0rn" ]; then
        time zombienet --provider="$provider" test $working_dir/smoke/9999-runtime_upgrade.zndsl
    else
        time zombienet --provider="$provider" test $working_dir/smoke/9999-runtime_upgrade_t1rn.zndsl
    fi
    echo "::endgroup::"
    echo "✅ Upgrade tests succeed!"
}

confirm_sfx() {
    docker-compose up -d grandpa-ranger
    echo "Spawning zombienet in the background..."
    zombienet --provider="$provider" test $working_dir/sfx/0000-confirm_sfx.zndsl
    docker-compose down
}

xcm() {
    echo "Running XCM tests.."
    echo "::group::Zombienet tests..."
    
    time zombienet --provider="$provider" test $working_dir/xcm/xcm.zndsl
    
    echo "::endgroup::"
}


spawn_alles() {
    setup_alles
    echo "Spawning zombienet for t0rn, t1rn, t7rn, t3rn using provider: $provider..."
    zombienet --provider="$provider" spawn ./zombienet-alles.toml
}

spawn_xcm() {
    setup_xcm
    echo "Spawning zombienet using provider: $provider..."
    zombienet --provider="$provider" spawn ./zombienet-xcm.toml
}

spawn() {
    setup
    echo "Spawning zombienet for ${NETWORK} using provider: $provider..."
    zombienet --provider="$provider" spawn ./zombienet-${NETWORK}.toml
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
        runtime_upgrade $@
    ;;
    "xcm")
        setup_xcm
        xcm
    ;;
    "spawn")
        setup
        spawn
    ;;
    "spawn_alles")
        setup
        spawn_alles
    ;;
    "spawn_xcm")
        spawn_xcm
    ;;
    "build")
        force_build_collator
    ;;
    *)
        echo "Enter an appropriate command"
        exit 1
    ;;
esac
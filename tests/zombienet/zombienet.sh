# bash shebang

set -e

root_dir=$(git rev-parse --show-toplevel)
bin_dir="$root_dir/bin"
working_dir="$(pwd)"

provider=native
zombienet_version=v1.3.43
pdot_branch=release-v0.9.37
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
    if [ "$(nixos-version 2>/dev/null)" == "" ]; then
        # Don't fetch zombienet if it's already present in the system
        if which zombienet-$zombienet_version >/dev/null; then
            cp $(which zombienet-$zombienet_version) "$bin_dir/zombienet"
            echo "✅ Zombienet $zombienet_version installed"
            return
        fi

        if [ ! -f "$bin_dir/zombienet" ]; then
            echo "Fetching zombienet..."
            curl -fL -o "$bin_dir/zombienet" "https://github.com/paritytech/zombienet/releases/download/$zombienet_version/zombienet-$machine"
            
            echo "Making zombienet executable"
            chmod +x "$bin_dir/zombienet"
        else 
            echo "✅ Zombienet $zombienet_version installed"
        fi
    else
        echo "This is a NixOS machine, skipping zombienet fetch"
        zombienet() {
            # nix run "github:paritytech/zombienet" $@
            node /home/common/projects/zombienet/javascript/packages/cli/dist/cli.js $@
        }
        export -f zombienet
    fi
}

build_polkadot() {
    # Don't compile polkadot if it's already present in the system
    if which polkadot-$pdot_branch >/dev/null; then
        cp $(which polkadot-$pdot_branch) "$bin_dir/polkadot"
        echo "✅ Polkadot $pdot_branch installed"
        return
    fi

    if [ ! -d "$polkadot_tmp_dir" ]; then
        echo "Cloning polkadot into $polkadot_tmp_dir"
        mkdir -p "$polkadot_tmp_dir"
        git clone --branch "$pdot_branch" --depth 1 https://github.com/paritytech/polkadot "$polkadot_tmp_dir/$pdot_branch"
    fi

    if [ ! -f "$polkadot_tmp_dir/$pdot_branch/target/release/polkadot" ]; then
        echo "Building polkadot..."
        cargo build --manifest-path "$polkadot_tmp_dir/$pdot_branch/Cargo.toml" --features fast-runtime --release --locked
    fi

    if [ ! -f "$bin_dir/polkadot" ]; then
        echo "Copying polkadot to bin dir"
        cp "$polkadot_tmp_dir/$pdot_branch/target/release/polkadot" "$bin_dir/polkadot"
    else
        echo "✅ Polkadot $pdot_branch installed"
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
    time cargo build --manifest-path "$root_dir/node/$NODE_ARG-parachain/Cargo.toml" --release --locked
    echo "::endgroup::"
    
    cp "$root_dir/target/release/$NODE_ARG-collator" "$bin_dir/"
}

setup() {
    make_bin_dir
    fetch_zombienet
    build_polkadot

    NODE_ARG=t0rn
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

upgrade() {
    if [[ $# -ne 2 ]]; then
        echo "Expecting exactly 2 arguments"
        echo $@
        echo "Usage: ./zombienet.sh upgrade <t3rn/t0rn>"
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
    echo "Spawning zombienet..."
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
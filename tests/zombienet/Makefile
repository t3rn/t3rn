.EXPORT_ALL_VARIABLES:

provider := native
version := v1.2.59
runtime := t3rn
pdot_branch := release-v0.9.27
root_dir := $(shell git rev-parse --show-toplevel)
bin_dir := $(root_dir)/bin
PATH := $(shell echo $$PATH):$(bin_dir)/
arch := $(shell sh -c 'uname -s 2>/dev/null || echo not')

ws_provider := wss://dev.net.t3rn.io

ifneq ($(findstring Darwin,$(arch)),)
	machine := macos
else ifneq ($(findstring Linux,$(arch)),)
	machine := linux
endif

mclean:
	rm -rf $(bin_dir)/*

# TODO: use makefile caching
${bin_dir}/zombienet:
	curl -fL -o $(bin_dir)/zombienet https://github.com/paritytech/zombienet/releases/download/$(version)/zombienet-$(machine)
	# echo "#### Need sudo access for zombienet executable ####"
	sudo chmod +x $(bin_dir)/zombienet

# TODO: use makefile caching
${bin_dir}/polkadot:
	tmp_dir=$(shell mktemp -d)
	git clone --branch $(pdot_branch) --depth 1 https://github.com/paritytech/polkadot $(tmp_dir)
	cargo build --manifest-path $(tmp_dir)/Cargo.toml --features fast-runtime --release --locked
	mv -f $(tmp_dir)/target/release/polkadot $(bin_dir)/polkadot

${bin_dir}/t0rn-collator:
	cargo build --manifest-path $(root_dir)/node/t0rn-parachain/Cargo.toml --release --locked
	cp -f $(root_dir)/target/release/t0rn-collator $(bin_dir)/

${bin_dir}/t3rn-collator:
	cargo build --manifest-path $(root_dir)/node/t3rn-parachain/Cargo.toml --release --locked
	cp -f $(root_dir)/target/release/t3rn-collator $(bin_dir)/

setup: ${bin_dir}/zombienet ${bin_dir}/polkadot ${bin_dir}/t0rn-collator ${bin_dir}/t3rn-collator
#    cargo build --manifest-path $root_dir/node/$runtime-parachain/Cargo.toml --release --locked
#    cp -f $root_dir/target/release/$runtime-collator $root_dir/bin/circuit-collator

${bin_dir}/subwasm:
	cargo install --locked --git https://github.com/chevdor/subwasm --tag v0.16.1 --path ${bin_dir}/

runtime-upgrade-blob: ${bin_dir}/subwasm
	# TODO: bump version of both parachains
	#new_spec_version=$(cat $root_dir/runtime/${parachain_name}-parachain/src/lib.rs | grep -o 'spec_version: [0-9]*' | tail -1 | grep -o '[0-9]')
	#new_impl_version=$(cat $root_dir/runtime/${parachain_name}-parachain/src/lib.rs | grep -o 'impl_version: [0-9]*' | tail -1 | grep -o '[0-9]')
	#new_tx_version=$(cat $root_dir/runtime/${parachain_name}-parachain/src/lib.rs | grep -o 'transaction_version: [0-9]*' | tail -1 | grep -o '[0-9]')
	#new_author_version=$(cat $root_dir/runtime/${parachain_name}-parachain/src/lib.rs | grep -o 'authoring_version: [0-9]*' | tail -1 | grep -o '[0-9]')

	# used_wasm=$(root_dir)/target/release/wbuild/t0rn-parachain-runtime/t0rn_parachain_runtime.compact.compressed.wasm
	# TODO: no need for subwasm i think
	# echo "🔢 hashing ${parachain_name}_parachain_runtime.compact.compressed.wasm..."
	# hash=$(subwasm info --json $used_wasm | jq -r .blake2_256)

test-smoke: ${bin_dir}/zombienet ${bin_dir}/polkadot
	# TODO: reuse the same network for the test
	# zombienet --provider=$(provider) test ./smoke/0001-is_up_and_registered.feature

test-upgrade:
	# get versions and bump t0rn
	cat $(root_dir)/runtime/t0rn-parachain/src/lib.rs | grep -o 'spec_version: [0-9]*' | tail -1 | grep -o '[0-9]' | xargs -I {} ./increment.sh t0rn {} spec_version
	cat $(root_dir)/runtime/t0rn-parachain/src/lib.rs | grep -o 'impl_version: [0-9]*' | tail -1 | grep -o '[0-9]' | xargs -I {} ./increment.sh t0rn {} impl_version
	cat $(root_dir)/runtime/t0rn-parachain/src/lib.rs | grep -o 'transaction_version: [0-9]*' | tail -1 | grep -o '[0-9]' | xargs -I {} ./increment.sh t0rn {} transaction_version
	cat $(root_dir)/runtime/t0rn-parachain/src/lib.rs | grep -o 'authoring_version: [0-9]*' | tail -1 | grep -o '[0-9]' | xargs -I {} ./increment.sh t0rn {} authoring_version
	
	# get versions and bump t3rn
	cat $(root_dir)/runtime/t3rn-parachain/src/lib.rs | grep -o 'spec_version: [0-9]*' | tail -1 | grep -o '[0-9]' | xargs -I {} ./increment.sh t3rn {} spec_version
	cat $(root_dir)/runtime/t3rn-parachain/src/lib.rs | grep -o 'impl_version: [0-9]*' | tail -1 | grep -o '[0-9]' | xargs -I {} ./increment.sh t3rn {} impl_version
	cat $(root_dir)/runtime/t3rn-parachain/src/lib.rs | grep -o 'transaction_version: [0-9]*' | tail -1 | grep -o '[0-9]' | xargs -I {} ./increment.sh t3rn {} transaction_version
	cat $(root_dir)/runtime/t3rn-parachain/src/lib.rs | grep -o 'authoring_version: [0-9]*' | tail -1 | grep -o '[0-9]' | xargs -I {} ./increment.sh t3rn {} authoring_version
	
	# build new blobs
	cargo build --manifest-path $(root_dir)/node/t0rn-parachain/Cargo.toml --release --locked
	cargo build --manifest-path $(root_dir)/node/t3rn-parachain/Cargo.toml --release --locked

	# deploy with test (ensuring old binary, new blob)
	zombienet --provider=$(provider) test ./smoke/9999-runtime_upgrade.feature

# This should only be run if the spec version is different to the old	
test-real-upgrade:
	# build new blobs
	cargo build --manifest-path $(root_dir)/node/t0rn-parachain/Cargo.toml --release --locked
	cargo build --manifest-path $(root_dir)/node/t3rn-parachain/Cargo.toml --release --locked

	# deploy with test (ensuring old binary, new blob)
	zombienet --provider=$(provider) test ./smoke/9999-runtime_upgrade.feature


test: test-smoke test-upgrade
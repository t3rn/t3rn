.EXPORT_ALL_VARIABLES:

provider := native
version := v1.2.59
runtime := t3rn
pdot_branch := release-v0.9.27
root_dir := $(shell git rev-parse --show-toplevel)
bin_dir := $(root_dir)/bin
PATH := $(shell echo $$PATH):$(bin_dir)/

arch := $(shell sh -c 'uname -s 2>/dev/null || echo not')

ifneq ($(findstring Darwin,$(arch)),)
	machine := macos
else ifneq ($(findstring Linux,$(arch)),)
	machine := linux
endif

clean:
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

test: ${bin_dir}/zombienet ${bin_dir}/polkadot
	zombienet --provider=$(provider) test ./smoke/0001-is_up_and_registered.feature
	
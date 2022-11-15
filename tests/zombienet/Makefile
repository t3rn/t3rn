.EXPORT_ALL_VARIABLES:

provider := native
version := v1.3.17
runtime := t3rn
pdot_branch := release-v0.9.27
root_dir := $(shell git rev-parse --show-toplevel)
bin_dir := $(root_dir)/bin
export PATH := $(shell echo $$PATH):$(bin_dir)
arch := $(shell sh -c 'uname -s 2>/dev/null || echo not')
tmp_dir := $(shell mktemp -d)

SCCACHE_VERSION := v0.3.1
SCCACHE_FILE := sccache-$(SCCACHE_VERSION)-x86_64-unknown-linux-musl
RUSTC_WRAPPER=sccache
ifneq ($(findstring Darwin,$(arch)),)
	machine := macos
else ifneq ($(findstring Linux,$(arch)),)
	machine := linux
endif

# ====================== Setup ======================

clean:
	rm -rf $(bin_dir)/*

directory:
	mkdir -p $(bin_dir)

${bin_dir}/sccache:
	@if [ $(machine) = "macos" ]; then \
        brew update; \
		brew install sccache; \
	elif [ $(machine) = "linux" ]; then \
		curl -fL -o $(tmp_dir)/$(SCCACHE_FILE).tar.gz https://github.com/mozilla/sccache/releases/download/$(SCCACHE_VERSION)/$(SCCACHE_FILE).tar.gz; \
		tar xzvf $(tmp_dir)/$(SCCACHE_FILE).tar.gz -C $(tmp_dir); \
		mv -f $(tmp_dir)/$(SCCACHE_FILE)/sccache $(bin_dir)/sccache; \
		sudo chmod +x $(bin_dir)/sccache; \
	fi
	
${bin_dir}/zombienet:
	curl -fL -o $(bin_dir)/zombienet https://github.com/paritytech/zombienet/releases/download/$(version)/zombienet-$(machine)
	# echo "#### Need sudo access for zombienet executable ####"
	sudo chmod +x $(bin_dir)/zombienet

${bin_dir}/polkadot:
	git clone --branch $(pdot_branch) --depth 1 https://github.com/paritytech/polkadot $(tmp_dir)
	cargo build --manifest-path $(tmp_dir)/Cargo.toml --features fast-runtime --release --locked
	mv -f $(tmp_dir)/target/release/polkadot $(bin_dir)/polkadot

${bin_dir}/t0rn-collator: $(wildcard $(root_dir)/*/t0rn-parachain/src/*.rs) $(root_dir)/*/t0rn-parachain/Cargo.toml Makefile
	cargo build --manifest-path $(root_dir)/node/t0rn-parachain/Cargo.toml --release --locked
	cp -f $(root_dir)/target/release/t0rn-collator $(bin_dir)/

${bin_dir}/t3rn-collator: $(wildcard $(root_dir)/*/t3rn-parachain/src/*.rs) $(root_dir)/*/t3rn-parachain/Cargo.toml Makefile
	cargo build --manifest-path $(root_dir)/node/t3rn-parachain/Cargo.toml --release --locked
	cp -f $(root_dir)/target/release/t3rn-collator $(bin_dir)/
	
${bin_dir}/subwasm:
	cargo install --locked --git https://github.com/chevdor/subwasm --tag v0.16.1 --path ${bin_dir}/

setup: directory ${bin_dir}/sccache ${bin_dir}/zombienet ${bin_dir}/polkadot ${bin_dir}/t0rn-collator ${bin_dir}/t3rn-collator

# ====================== Caching ======================

start-sccache: ${bin_dir}/sccache
	sccache --start-server

stop-sccache: ${bin_dir}/sccache
	sccache --stop-server || true
	
print-sccache: ${bin_dir}/sccache
	sccache --show-stats
	
# ====================== Testing ======================

test-smoke: setup
	# FIXME: ensure this runs on all changes
	# TODO[Optimisation]: loop through directory and test all
	# TODO[Optimisation]: when zombienet can run on a pre-existing network, run it
	zombienet --provider=$(provider) test ./smoke/0001-is_up_and_registered.feature

test-upgrade: setup
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

# This should only be run if the spec version is different to the old, simply write the runtime to test to the same file and the test will check it
test-real-upgrade: setup
	# build new blobs
	cargo build --manifest-path $(root_dir)/node/t0rn-parachain/Cargo.toml --release --locked
	cargo build --manifest-path $(root_dir)/node/t3rn-parachain/Cargo.toml --release --locked

	# deploy with test (ensuring old binary, new blob)
	zombienet --provider=$(provider) test ./smoke/9999-runtime_upgrade.feature


test: test-smoke test-upgrade
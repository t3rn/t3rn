.EXPORT_ALL_VARIABLES:

provider := native
zombienet_version := v1.3.17
runtime := t3rn
pdot_branch := release-v0.9.37
root_dir := $(shell git rev-parse --show-toplevel)
bin_dir := $(root_dir)/bin
arch := $(shell sh -c 'uname -s 2>/dev/null || echo not')
polkadot_tmp_dir := /tmp/polkadot
SCCACHE_VERSION := v0.3.1
SCCACHE_FILE := sccache-$(SCCACHE_VERSION)-x86_64-unknown-linux-musl

export PATH := $(shell echo $$PATH):$(bin_dir)

ifneq ($(findstring Darwin,$(arch)),)
	machine := macos
else ifneq ($(findstring Linux,$(arch)),)
	machine := linux
endif

# ====================== Setup ======================

clean:
	rm -rf $(bin_dir)/*

$(bin_dir): 
	@mkdir -p $(bin_dir)

${bin_dir}/zombienet:
	curl -fL -o $(bin_dir)/zombienet https://github.com/paritytech/zombienet/releases/download/$(zombienet_version)/zombienet-$(machine)
	chmod +x $(bin_dir)/zombienet

$(polkadot_tmp_dir)/$(pdot_branch)/target/release/polkadot:
	mkdir -p $(polkadot_tmp_dir);
	git clone --branch $(pdot_branch) --depth 1 https://github.com/paritytech/polkadot $(polkadot_tmp_dir)/$(pdot_branch);
	cargo build --manifest-path $(polkadot_tmp_dir)/$(pdot_branch)/Cargo.toml --features fast-runtime --release --locked;
	
${bin_dir}/polkadot: $(polkadot_tmp_dir)/$(pdot_branch)/target/release/polkadot
	cp $(polkadot_tmp_dir)/$(pdot_branch)/target/release/polkadot $(bin_dir)/polkadot

${bin_dir}/t3rn-collator: $(wildcard $(root_dir)/*/t3rn-parachain/src/*.rs) $(root_dir)/*/t3rn-parachain/Cargo.toml Makefile
	cargo build --manifest-path $(root_dir)/node/t3rn-parachain/Cargo.toml --release --locked
	cp -f $(root_dir)/target/release/t3rn-collator $(bin_dir)/

setup: $(bin_dir) ${bin_dir}/zombienet ${bin_dir}/polkadot ${bin_dir}/t3rn-collator

# ====================== Caching ======================
#
# sccache is already present on Github Action workers, no need to set it up
#

${bin_dir}/sccache:
	@if [ $(machine) = "macos" ]; then \
        brew update; \
		brew install -qf sccache | true; \
		cp -n /opt/homebrew/bin/sccache $(bin_dir)/sccache; \
	elif [ $(machine) = "linux" ]; then \
		tmp_dir=$(shell mktemp -d); \
		curl -fL https://github.com/mozilla/sccache/releases/download/$(SCCACHE_VERSION)/$(SCCACHE_FILE).tar.gz | tar xzvf - -C $(bin_dir)/; \
		mv $(bin_dir)/$(SCCACHE_FILE)/sccache $(bin_dir)/; \
		sudo chmod +x $(bin_dir)/sccache; \
	fi
	
start-sccache: $(bin_dir) ${bin_dir}/sccache
	sccache --start-server || true

stop-sccache: ${bin_dir}/sccache
	sccache --stop-server || true
	
print-sccache: ${bin_dir}/sccache
	sccache --show-stats
	
# ====================== Testing ======================

test-smoke: setup

	@if [ $(machine) = "macos" ]; then \
        echo "You're on macos, this is for CI :< - it uses previously built binaries from this machine only"; \
		exit 1; \
	fi

	# TODO[Optimisation]: loop through directory and test all
	# TODO[Optimisation, NotImplemented]: when zombienet can run on a pre-existing network, run it
	$(bin_dir)/zombienet --provider=$(provider) test ./smoke/0001-is_up_and_registered.feature

test-upgrade: $(bin_dir) ${bin_dir}/zombienet ${bin_dir}/polkadot
	@echo "Testing real upgrade for parachain: $(parachain)"

	@if [ $(machine) = "macos" ]; then \
        echo "You're on macos, this is for CI :< - it uses previously built binaries from this machine only"; \
		exit 1; \
	fi

	@echo get last release binary from github
	git fetch --all --tags -f || true

	./download.sh $(parachain)

	@echo "deploy with test (ensuring old binary, new blob)"
	zombienet --provider=$(provider) test ./smoke/9999-runtime_upgrade.feature

spawn: setup
	$(bin_dir)/zombienet --help
	$(bin_dir)/zombienet --provider=$(provider) spawn ./zombienet.toml

test: test-smoke

pull-request: test

release: test-upgrade
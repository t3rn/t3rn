.EXPORT_ALL_VARIABLES:

dir := parachains-integration-tests
url := https://github.com/paritytech/$(dir).git
# relative to $(dir)...
test_path := "../tests"
config_path := "../tests"
zombienet_config := "../../zombienet/zombienet.toml"
opts := --chain-logs "../chain.log" --test-logs "../test.log"
# from zombienet's Makefile
provider := native
zombienet_version := v1.3.17
runtime := t3rn
pdot_branch := release-v0.9.27
root_dir := $(shell git rev-parse --show-toplevel)
bin_dir := $(root_dir)/bin
arch := $(shell sh -c 'uname -s 2>/dev/null || echo not')
polkadot_tmp_dir := /tmp/polkadot

export PATH := $(shell echo $$PATH):$(bin_dir)

ifneq ($(findstring Darwin,$(arch)),)
	machine := macos
else ifneq ($(findstring Linux,$(arch)),)
	machine := linux
endif

# ====================== Setup ======================

clean:
	@echo "🧼 Cleaning..."
	@rm -rf $(dir)
	@rm -rf $(bin_dir)/*

$(dir):
	@echo "⬇ Getting repo..."
	@git clone $(url)

$(dir)/dist: $(dir)
	@echo "🔧 Checking tool..."
	@cd $(dir); \
	yarn

$(bin_dir): 
	@mkdir -p $(bin_dir)

${bin_dir}/zombienet:
	curl -fL -o $(bin_dir)/zombienet https://github.com/paritytech/zombienet/releases/download/$(zombienet_version)/zombienet-$(machine)
	chmod +x $(bin_dir)/zombienet

${bin_dir}/polkadot:
	@if [ ! -f $(polkadot_tmp_dir)/$(pdot_branch)/target/release/polkadot ]; then \
		mkdir -p $(polkadot_tmp_dir); \
		git clone --branch $(pdot_branch) --depth 1 https://github.com/paritytech/polkadot $(polkadot_tmp_dir)/$(pdot_branch); \
		cargo build --manifest-path $(polkadot_tmp_dir)/$(pdot_branch)/Cargo.toml --features fast-runtime --release --locked; \
	fi
	cp $(polkadot_tmp_dir)/$(pdot_branch)/target/release/polkadot $(bin_dir)/polkadot

${bin_dir}/t0rn-collator: $(wildcard $(root_dir)/*/t0rn-parachain/src/*.rs) $(root_dir)/*/t0rn-parachain/Cargo.toml Makefile
	cargo build --manifest-path $(root_dir)/node/t0rn-parachain/Cargo.toml --release --locked
	cp -f $(root_dir)/target/release/t0rn-collator $(bin_dir)/

${bin_dir}/t3rn-collator: $(wildcard $(root_dir)/*/t3rn-parachain/src/*.rs) $(root_dir)/*/t3rn-parachain/Cargo.toml Makefile
	cargo build --manifest-path $(root_dir)/node/t3rn-parachain/Cargo.toml --release --locked
	cp -f $(root_dir)/target/release/t3rn-collator $(bin_dir)/

setup: $(dir) $(dir)/dist $(bin_dir) ${bin_dir}/zombienet ${bin_dir}/polkadot ${bin_dir}/t0rn-collator ${bin_dir}/t3rn-collator

# ====================== Testing ======================

check: setup 
	@echo "✅ Set up DONE!"
	@echo "⏳ Started checker..."
	@cd $(dir); \
	yarn checker -t $(test_path);

test: setup
	@echo "✅ Set up DONE!"
	@echo "⏳ Started testing..."
	@cd $(dir); \
	yarn test -t $(test_path)

test-logs: setup
	@echo "✅ Set up DONE!"
	@echo "⏳ Started testing (check logs in file)..."
	@cd $(dir); \
	yarn test -t $(test_path) $(opts)

ctest: clean setup
	@echo "✅ Set up and clean DONE!"
	@echo "⏳ Started testing..."
	@cd $(dir); \
	yarn test -t $(test_path) $(opts)

ctest-logs: clean setup
	@echo "✅ Set up and clean DONE!"
	@echo "⏳ Started testing (check logs in file)..."
	@cd $(dir); \
	yarn test -t $(test_path)

zombie: setup
	@echo "✅ Set up DONE!"
	@echo "⏳ Started zombienet testing..."
	@cd $(dir); \
	yarn zombienet -c $(zombienet_config)

zombie-logs: setup
	@echo "✅ Set up DONE!"
	@echo "⏳ Started zombienet testing (check logs in file)..."
	@cd $(dir); \
	yarn zombienet -c $(zombienet_config) $(opts)

zombie-test: setup
	@echo "✅ Set up DONE!"
	@echo "⏳ Started zombienet testing..."
	@cd $(dir); \
	yarn checker -t $(test_path); \
	yarn zombienet-test -t $(test_path) -c $(zombienet_config)

zombie-test-logs: setup
	@echo "✅ Set up DONE!"
	@echo "⏳ Started zombienet testing (check logs in file)..."
	@cd $(dir); \
	yarn checker -t $(test_path); \
	yarn zombienet-test -t $(test_path) -c $(zombienet_config) $(opts)

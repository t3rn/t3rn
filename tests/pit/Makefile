.EXPORT_ALL_VARIABLES:

PATH := $(shell echo $$PATH):$(shell pwd)/node_modules/.bin/:$(shell pwd)/../../bin/

setup:
	yarn install
	../../zombienet.sh

clean-logs:
	rm -rf logs

clean-bins:
	killall polkadot &
	killall t3rn-collator &
	killall t0rn-collator &

clean: clean-logs clean-bins
	rm -rf node_modules

test: clean-logs clean-bins
	@printf "\n🔍  Running tests...\n\n"

	mkdir -p logs


	cd ../../ && \
 		parachains-integration-tests \
			-m zombienet \
			-c zombienet.toml \
			-cl tests/pit/logs/chains.log 2> /dev/null &

	parachains-integration-tests \
		-m test \
		-t ./files \
		-tl ./logs/tests.log

	tail -n 100 logs/tests.log

	printf "\n🎉 integration tests finished! \n\n"
all:
	cd types; yarn; yarn link; yarn build;
	cd sdk; yarn; yarn link; yarn build;
	cd cli; yarn add ../sdk; yarn;
	cd executor; pnpm i; pnpm add ../sdk; pnpm add ../types;
	cd attester; pnpm i; pnpm add ../types; pnpm add ../sdk;

executor:
	cd executor; pnpm i; pnpm add ../sdk; pnpm add ../types;

attester:
	cd attester; pnpm i; pnpm add ../types; pnpm add ../sdk;

sdk:
	cd sdk; yarn; yarn link; yarn build;

types:
	cd types; yarn; yarn link; yarn build;

clean:
	find . -type d -name node_modules | xargs rm -rf {}\;

test:
	cd cli; pnpm run test
	cd executor; yarn test

register_roco:
	cd cli; ts-node index.ts register roco;

submit_headers_roco:
	cd cli; ts-node  index.ts submit-headers roco;

submit_headers_roco_x3:
	for i in {1..3} ; do sleep 12; cd cli; ts-node index.ts submit-headers roco; done

submit_side_effects:
	cd cli; ts-node index.ts submit-side-effects config/transfer.ts;

start_executor_nohup:
	cd executor; yarn; nohup yarn start > /tmp/t3rn_packages_test_executor.out 2>&1 &

start_executor:
	cd executor; yarn; yarn start;

test_confirm_xtx: register_roco submit_headers_roco submit_side_effects submit_headers_roco_x3

.PHONY: all old sdk types attester executor

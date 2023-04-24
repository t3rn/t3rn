 all:
	cd types; yarn; yarn build;
	cd sdk; yarn; yarn build;
	cd cli; yarn; npm i;
	cd executor; yarn; npm i;

register_roco:
	cd cli; ts-node index.ts register roco;

submit_headers_roco:
	cd cli; ts-node  index.ts submit-headers roco;

submit_headers_roco_x3:
	for i in {1..3} ; do sleep 12; cd cli; ts-node index.ts submit-headers roco; done

submit_side_effects:
	cd cli; ts-node index.ts submit-side-effects config/transfer.ts;

start_executor:
	cd executor; yarn; nohup yarn start > /tmp/t3rn_packages_test_executor.out 2>&1 &

test_confirm_xtx: register_roco start_executor submit_headers_roco submit_side_effects submit_headers_roco_x3

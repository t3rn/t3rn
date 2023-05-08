 all:
	cd types; npm i; npm build;
	cd sdk; npm i; npm build;
	cd cli; npm add @t3rn/sdk; npm i;
	cd executor; npm add @t3rn/sdk; npm add @t3rn/types; npm upgrade @t3rn/sdk; npm upgrade @t3rn/types; npm i;

register_roco:
	cd cli; ts-node index.ts register roco;

submit_headers_roco:
	cd cli; ts-node  index.ts submit-headers roco;

submit_headers_roco_x3:
	for i in {1..3} ; do sleep 12; cd cli; ts-node index.ts submit-headers roco; done

submit_side_effects:
	cd cli; ts-node index.ts submit-side-effects config/transfer.ts;

start_executor_nohup:
	cd executor; npm; nohup npm start > /tmp/t3rn_packages_test_executor.out 2>&1 &

start_executor:
	cd executor; npm; npm start;

test_confirm_xtx: register_roco submit_headers_roco submit_side_effects submit_headers_roco_x3

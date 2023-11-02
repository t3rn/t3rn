export default {
	circuit: {
		rpc1: {
			ws: "ws://host.docker.internal:9944",
			http: "http://host.docker.internal:9944"
		},
		rpc2: {
			ws: "ws://host.docker.internal:9944",
			http: "http://host.docker.internal:9944"
		},
	},
	target: { // we dont need to specify the http endpoint for the target
		rpc1: {
			ws: "ws://host.docker.internal:9933",
		},
		rpc2: {
			ws: "ws://host.docker.internal:9933"
		},
	},
	rangeInterval: 10, // time between range submissions in seconds
	targetGatewayId: "roco",
	bridgeName: "rococoBridge",
	quickSyncLimit: 200, // for more than 200 blocks behind, use quick sync
	batches_max: 10,
}
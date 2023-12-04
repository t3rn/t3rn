export default {
	circuit: {
		rpc1: {
			ws: "ws://localhost:9944",
			http: "http://localhost:9944"
		},
		rpc2: {
			ws: "ws://localhost:9944",
			http: "http://localhost:9944"
		},
	},
	target: { // we dont need to specify the http endpoint for the target
		rpc1: {
			ws: "wss://rococo-rpc.polkadot.io",
		},
		rpc2: {
			ws: "wss://rococo-community-rpc.laminar.codes/ws"
		},
	},
	rangeInterval: 5, // time between range submissions in seconds
	targetGatewayId: "pdot",
	bridgeName: "polkadotBridge",
	batches_max: 10,
	quickSyncLimit: 0, // for more than 200 blocks behind, use quick sync
}
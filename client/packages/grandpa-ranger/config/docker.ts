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
			ws: "wss://rococo-rpc.polkadot.io",
		},
		rpc2: {
			ws: "wss://rococo-community-rpc.laminar.codes/ws"
		},
	},
	rangeInterval: 10, // time between range submissions in seconds
	targetGatewayId: "roco",
}
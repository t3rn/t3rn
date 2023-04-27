export default {
	circuit: {
		rpc1: "ws://localhost:9944",
		rpc2: "ws://localhost:52417",
		signer: "0x0177d124e501887c2470e260c8f0da60db9ed3dba808a682f09afb39eff0c561",
	},
	target: {
		rpc1: "wss://rococo-rpc.polkadot.io",
		rpc2: "wss://rococo-community-rpc.laminar.codes/ws",
	},
	rangeBreak: 180, // time between range submissions in seconds
}
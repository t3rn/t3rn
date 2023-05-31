export default {
    circuit: {
        rpc1: {
            ws: 'wss://ws.t0rn.io',
            http: 'https://rpc.t0rn.io',
        },
    },
    target: {
        // we dont need to specify the http endpoint for the target
        rpc1: {
            ws: 'wss://rococo-rpc.polkadot.io',
        },
        rpc2: {
            ws: 'wss://rococo-community-rpc.laminar.codes/ws',
        },
    },
}

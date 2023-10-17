const { ApiPromise, WsProvider, Keyring } = require("@polkadot/api")



async function assertEnv() {
    if (
        !process.env.PROVIDER.startsWith("ws://") &&
        !process.env.PROVIDER.startsWith("wss://")
    ) {
        throw Error(`invalid env var PROVIDER ${process.env.PROVIDER}`)
    }
    if (!process.env.SUBMITTER) {
        throw Error(`invalid env var SUBMITTER ${process.env.SUBMITTER}`)
    }
}
async function main() {

    await assertEnv()
    // local provider: PROVIDER=ws://localhost:9944
    const provider = new WsProvider(process.env.PROVIDER)
    const circuit = await ApiPromise.create({ provider })
    const keyring = new Keyring({ type: "sr25519" })
    // local alice SUBMITTER=//Alice
    const submitter = keyring.addFromMnemonic(process.env.SUBMITTER)

    const vacuumOrder =
        circuit.tx.vacuum.order([{
            sfx_action: {"Transfer": ["0x03030303", "0x000000bb", "t3XVH2FfTnHCEJDmfj8zBmm5UTV26soZfdW7GFpuJ3ba2Rt42", 100]},
            max_reward: 100,
            reward_asset: "0x000000bb",
            insurance: 100,
            remote_origin_nonce: 0,
        } ], 2)

    const vacuumOrderSingle =
        circuit.tx.vacuum.singleOrder(
            "0x03030303",
            "0x000000bb", // 187
            100,
            "0x000000bb", // 187
            200,
            20,
            "5C57fbAUMpvrogfFCXsgsZLY4eZyjGkjV8MGHpK6tNYofzJo",
            2,
        )

    // const reboot = await circuit.tx.xdns.rebootSelfGateway(0)
    // const _rebootTxHash = await circuit.tx.sudo.sudo(reboot).signAndSend(submitter);

    const { nonce, data: balance } = await circuit.query.system.account(submitter.address);

    for (const index of [0, 1, 2, 3, 4, 5, 6, 7, 8, 9,]) {
        console.log("vacuumOrder.signAndSend with nonce", nonce.toNumber() + index)
        await vacuumOrderSingle.signAndSend(submitter, {nonce: nonce.toNumber() + index});
    }

    // console.log(txHash)
    circuit.disconnect()
}

main()
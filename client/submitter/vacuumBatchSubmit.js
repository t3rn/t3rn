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

    const maybePeriodic = null
    const schedulePriority = 0

    /*
        #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
        pub enum SFXAction<Account, Asset, Balance, Destination, Input, MaxCost> {
            // All sorts of calls: composable, wasm, evm, etc. are vacuumed into a single Call SFX in the protocol level.
            Call(Destination, Account, Balance, MaxCost, Input),
            // All of the DEX-related SFXs are vacuumed into a Transfer SFX in the protocol level: swap, add_liquidity, remove_liquidity, transfer asset, transfer native
            Transfer(Destination, Asset, Account, Balance),
        }

        #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
        pub struct OrderSFX<AccountId, Asset, Balance, Destination, Input, MaxCost> {
            pub sfx_action: SFXAction<AccountId, Asset, Balance, Destination, Input, MaxCost>,
            pub max_reward: Balance,
            pub reward_asset: Asset,
            pub insurance: Balance,
            pub remote_origin_nonce: Option<u32>,
        }
     */

    const vacuumOrder =
        circuit.tx.vacuum.order([{
            sfx_action: {"Transfer": ["0x03030303", "0x0000aabb", "t3XVH2FfTnHCEJDmfj8zBmm5UTV26soZfdW7GFpuJ3ba2Rt42", 100]},
            max_reward: 100,
            reward_asset: "0x0000aabb",
            insurance: 100,
            remote_origin_nonce: 0,
        } ], 2)

    //           destination: TargetId,
    //             asset: Asset,
    //             amount: BalanceOf<T>,
    //             reward_asset: Asset,
    //             max_reward: BalanceOf<T>,
    //             insurance: BalanceOf<T>,
    //             target_account: T::AccountId,
    //             speed_mode: SpeedMode,
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

    //
    // const reboot = await circuit.tx.xdns.rebootSelfGateway(0)
    //
    // const _rebootTxHash = await circuit.tx.sudo.sudo(reboot).signAndSend(submitter);
    //
    const { nonce, data: balance } = await circuit.query.system.account(submitter.address);

    // generate 100 000 number array
    const one_hundred_tausend = Array.from(Array(1000).keys())

    for (const index of one_hundred_tausend) {


        console.log("vacuumOrder.signAndSend with nonce", nonce.toNumber() + index)
        await vacuumOrderSingle.signAndSend(submitter, {nonce: nonce.toNumber() + index});
        // const txHash = await new Promise((resolve) => {
        //     vacuumOrderSingle.signAndSend(submitter, {nonce: nonce.toNumber() + index}, ({ status, events }) => {
        //         console.log(`Current status is ${status.type}`);
        //
        //         if (status.isInBlock) {
        //             console.log("Transaction is in a block!");
        //             resolve(status.asInBlock.toHex());
        //         }
        //
        //         console.log(`Transaction status: ${status}`)
        //
        //     });
        // });
    }

    // console.log(txHash)
    circuit.disconnect()
}

main()
// import "@t3rn/types/api-augment"

import { ApiPromise, WsProvider } from '@polkadot/api';
import { register, setOperational } from "./register";
import types from "./types.json"
// import { submitTransfer } from "./submit";
import { SubstrateListener } from './listener';


class TransferSiseEffect {
    listener: SubstrateListener
    rococo: ApiPromise;
    circuit: ApiPromise;
    target: number[];

    async setup() {
        this.target = [97, 98, 99, 100].map(() => Math.floor(97 + Math.random() * 26));
        this.listener = new SubstrateListener(this.circuit, this.rococo, this.target)

        this.rococo = await ApiPromise.create({ 
            provider: new WsProvider("wss://rococo-rpc.polkadot.io"),
        })
        this.circuit = await ApiPromise.create({
            provider: new WsProvider("ws://127.0.0.1:9944"),
            types: types as any
        })
    }

    async run() {
        // 
        await this.setup();
        console.log("Initialized API")
        await register(this.circuit, this.target)
        await this.delay()
        console.log("Registered Roccoco")
        await setOperational(this.circuit, this.target)
        // await this.listener.test(this.target)
        await this.listener.initListener()
        // await submitTransfer(this.api, this.target)
        // console.log("Submitted Transfer")
        // this.api.disconnect()
    }

    // async initEventListener() : Promise<void> {
    //     const api = await ApiPromise.create({
    //         provider: new WsProvider("wss://rococo-rpc.polkadot.io"),
    //     })

    //     const circuit = await ApiPromise.create({
    //         provider: new WsProvider("ws://127.0.0.1:9944"),
    //     })

    //     const subJust = await api.rpc.grandpa.subscribeJustifications(async res => {
    //         console.log("Justification received")
    //         let tx = await circuit.tx.multiFinalityVerifierPolkadotLike.submitFinalityProof(res.toHuman())

    //         const keyring = new Keyring({ type: 'sr25519', ss58Format: 60 });
    //         const alice = keyring.addFromUri('//Alice');
    //         circuit.tx.sudo.sudoUncheckedWeight(tx, 1).signAndSend(alice)
    //         .then(res => console.log("res:", res.toHuman()))
            
    //         subJust()
    //     });
    // }

    async delay() {
        return new Promise<void>((res, rej) => {
            setTimeout(() => {
                res()
            }, 6000)
        })
    }

}


(async () => {
    let trans = new TransferSiseEffect();
    trans.run()
})()


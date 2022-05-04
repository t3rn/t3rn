// import "@t3rn/types/api-augment"

import { ApiPromise, WsProvider } from '@polkadot/api';
import { register, setOperational, registerParachain, getStorageKey } from "./register";
import types from "./types.json"
import { submitTransfer } from "./submit";
import { submitOptimisticTransfer } from './submit_optimistic';
import { SubstrateListener } from './listener';


class TransferSiseEffect {
    listener: SubstrateListener
    rococo: ApiPromise;
    circuit: ApiPromise;
    rococoId: number[];
    transactionTarget: number[];
    dolphinId: number[];

    async setup() {
        this.rococoId = [ 114, 111, 99, 111 ]
        this.transactionTarget = [ 114, 111, 99, 111 ];
        this.dolphinId = [ 100, 111, 108, 112 ];
        // .map(() => Math.floor(97 + Math.random() * 26));
        
        this.rococo = await ApiPromise.create({ 
            provider: new WsProvider("wss://rococo-rpc.polkadot.io"),
        })
        this.circuit = await ApiPromise.create({
            provider: new WsProvider("ws://127.0.0.1:9944"),
            types: types as any
        })

        this.listener = new SubstrateListener(this.circuit, this.rococo, this.rococoId)
    }

    async close() {
        this.rococo.disconnect();
        this.circuit.disconnect();
    }

    async run() {
        await this.setup();
        console.log("Initialized API")
        const args = process.argv[2]
        switch(args) {
            case "register_relay": {
                await this.registerRelaychain()
                break;
            }
            case "register_parachain": {
                await this.registerParachain()
                break;
            }
            case "submit_finality_proof": {
                this.listener.initListener()
                break;
            }
            case "submit_transfer": {
                await submitTransfer(this.circuit, this.transactionTarget);
                break;
            }
            case "submit_optimistic_transfer": {
                await submitOptimisticTransfer(this.circuit, this.transactionTarget);
                break;
            }
            case "submit_para_header": {
                await getStorageKey(this.rococo);
                break;
            }
        }
    }

    async registerRelaychain() {
        await register(this.circuit, this.rococoId)
        await this.delay()
        console.log("Registered Roccoco")
        await setOperational(this.circuit, this.rococoId)
        console.log("Set operational")
    }

    async registerParachain() {
        await registerParachain(this.circuit, this.dolphinId)
        await this.delay()
        await setOperational(this.circuit, this.dolphinId)
        console.log("Registered Dolphin")
    }

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


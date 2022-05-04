import { EventEmitter } from 'events'
import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { SideEffect } from '../utils/types';
import { decodeSfxArgs } from './utils/helpers';
import chalk from 'chalk';

export default class CircuitRelayer extends EventEmitter {

    api: ApiPromise;
    id: string;
    rpc: string;
    signer: any;
    // color: string;

    log(msg: string) {
        // console.log(chalk[this.color]("Circuit - "), msg)
        console.log("Circuit - ", msg)
    }

    async setup(rpc: string){//, color: string) {
        this.rpc = rpc;
        this.api = await ApiPromise.create({
            provider: new WsProvider(rpc),
        })

        // this.color = color;
        
        const keyring = new Keyring({ type: 'sr25519' });

        this.signer =
            process.env.CIRCUIT_KEY === undefined
                ? keyring.addFromUri('//Alice')
                : keyring.addFromMnemonic(process.env.CIRCUIT_KEY);
    }

    async maybeBondInsuranceDeposit(sideEffect: SideEffect) {
        // const args = decodeSfxArgs(sideEffect)
        console.log("maybeBondInsuranceDeposits")

        if (sideEffect.object.encodedArgs.length === 4) {
            console.log("bonding insurance deposit...")
            await this.api.tx.circuit
            .bondInsuranceDeposit(sideEffect.xtxId, sideEffect.getId()) 
            .signAndSend(this.signer, result => {
                console.log("bond insurance deposit:", result.toHuman())
            })  
        }
    }

    async confirmSideEffects(sideEffects: SideEffect[]) {
        await Promise.all(
            sideEffects.map(sideEffect => this.confirmSideEffect(sideEffect))
        )
        this.log("Confirmed SideEffects: " + sideEffects.length)
    }

    async confirmSideEffect(sideEffect: SideEffect) {
        let tx = this.api.tx.circuit.confirmSideEffect(
            sideEffect.xtxId, 
            sideEffect.object, 
            sideEffect.confirmedSideEffect, 
            sideEffect.inclusionProof.toJSON().proof,
            sideEffect.execBlockHeader.toJSON()
        )

        return new Promise(async (res, rej) => {
            let unsub = await tx.signAndSend(this.signer, (result) => {
                if (result.status.isFinalized) {
                    console.dir(result.events.map(e => e.toHuman()),  { depth: null })
                    const success = result.events[result.events.length - 1].event.method === "ExtrinsicSuccess";
                    this.log(`SideEffect confirmed: ${success}, ${result.status.asFinalized}`)
                    sideEffect.confirm(success, result.status.asFinalized)
    
                    this.emit(
                        "SideEffectConfirmed",
                        sideEffect.getId()
                    )
                    
                    res(unsub());
                }
            });
        })


    }
}
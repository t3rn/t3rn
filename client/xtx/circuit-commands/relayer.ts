import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { JustificationNotification } from '@polkadot/types/interfaces/';
import types from "./types.json"
import { getHeaderProof } from "./utils/helpers";

export const submitProof = async (justification: any, header: any, gatewayId: any[], rococo: any) => {
    let circuit = await ApiPromise.create({
        provider: new WsProvider("ws://127.0.0.1:9944"),
        types: types as any
    })
    let tx = await circuit.tx.multiFinalityVerifierDefault.submitFinalityProof(
        header,
        justification,
        gatewayId
    )

    console.log("Submitting finality proof");

    const keyring = new Keyring({ type: 'sr25519', ss58Format: 60 });
    const alice = keyring.addFromUri('//Alice');
    // await tx.signAndSend(alice)

    await tx.signAndSend(alice, async (result) => {
        if (result.status.isFinalized) {
            console.log("Submitting header proof");
            console.log("Rococo header:", header.hash)
            console.log("Rococo header:", header.toHuman())
            const [headerProof, key] = await getHeaderProof(rococo, header.hash, 2000)
            await submitHeaderProof(headerProof, [97, 98, 99, 100], [109, 111, 111, 110], key, header.hash)
        }   
    })

    // this.delay();
        // await submitHeaderProof(headerProof, [97, 98, 99, 100], [109, 111, 111, 110], paraHeader)

}

export const submitHeaderProof = async (proof: any, relayId: number[], gatewayId: number[], key: any, header: any) => {
    let circuit = await ApiPromise.create({
        provider: new WsProvider("ws://127.0.0.1:9944"),
        types: types as any
    })

    let tx = await circuit.tx.circuitPortal.submitParachainHeader(
        header,
        gatewayId,
        proof.proof
    )

    //  relay_chain_id: ChainId,
    //         block_hash: Vec<u8>,
    //         header_key: Vec<u8>,
    //         proof: StorageProof,

    const keyring = new Keyring({ type: 'sr25519', ss58Format: 60 });
    const alice = keyring.addFromUri('//Alice');
    return tx.signAndSend(alice, async (result) => {
        if (result.status.isFinalized) {
            console.log("Submitted header proof!");
            console.log(result)
        }
    })
}
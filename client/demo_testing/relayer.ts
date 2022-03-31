import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { JustificationNotification } from '@polkadot/types/interfaces/';
import types from "./types.json"

export const submitProof = async (justification: any, header: any, gatewayId: any[]) => {
    let circuit = await ApiPromise.create({
        provider: new WsProvider("ws://127.0.0.1:9944"),
        types: types as any
    })
    let tx = await circuit.tx.multiFinalityVerifierDefault.submitFinalityProof(
        header,
        justification,
        gatewayId
    )

    const keyring = new Keyring({ type: 'sr25519', ss58Format: 60 });
    const alice = keyring.addFromUri('//Alice');
    await tx.signAndSend(alice)
}
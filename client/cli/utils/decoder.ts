import { TypeRegistry, createType } from "@polkadot/types"
import{ Keyring }from'@polkadot/api';
const keyring = new Keyring({ type: "sr25519" })

const registry = new TypeRegistry()
const justification = { type: 'GrandpaJustification<Header>' }
const finalityProof = { proof: "(Header::Hash, Vec<u8>, Vec<Header>)" }
const header = { type: 'Header' }

export const decodeFinalityProof = (data: any) => {
    registry.register(finalityProof);

    const res = createType(registry, finalityProof.proof, data.toJSON()) // toJSON works, toHEX() not
    return {latestBlockHash: res[0], justification: res[1], headers: res[2]}
}

export const decodeJustification = (data: any) => {
    registry.register(justification);
    return createType(registry, justification.type as any, data)
}

export const decodeAuthoritySet = (data: any) => {
    const justification = decodeJustification((data))
    return justification.commit.precommits.map(entry => entry.id.toHex()).sort();
}

export const extractAuthoritySetFromFinalityProof = (finalityProof: any) => {
    const rawJust = decodeFinalityProof(finalityProof).justification
    return decodeAuthoritySet(rawJust)
}

export const decodeHeader = (data: string) => {
    registry.register(header);
    return createType(registry, header.type as any, data)
}

export const addressStringToPubKey = (address: string) => {
    return "0x" + Buffer.from(keyring.decodeAddress(address)).toString('hex')
}
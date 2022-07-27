import { TypeRegistry, createType } from "@polkadot/types"

const registry = new TypeRegistry()
const justification = { type: 'GrandpaJustification<Header>' }

const finalityProof = { proof: "(Header::Hash, Vec<u8>, Vec<Header>)" }
export const decodeFinalityProof = (data: any) => {
    const registry = new TypeRegistry()
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
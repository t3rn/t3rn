import { TypeRegistry, createType } from '@polkadot/types'

const registry = new TypeRegistry()

const justification = { type: 'GrandpaJustification<Header>' }
const finalityProof = { proof: '(Header::Hash, Vec<u8>, Vec<Header>)' }
const header = { type: 'Header' }

/**
 * Decode the finality proof
 * @param data - The data to decode
 */

export const finalityProofDecode = (data: any) => {
  registry.register(finalityProof)

  const res = createType(registry, finalityProof.proof, data.toJSON()) // toJSON works, toHEX() not
  // @ts-ignore
  return { latestBlockHash: res[0], justification: res[1], headers: res[2] }
}

/**
 * Decode the header
 * @param data - The data to decode
 */

export const justificationDecode = (data: any) => {
  registry.register(justification)

  return createType(registry, justification.type as any, data)
}

/**
 * Decode the authority set
 * @param data - The data to decode
 */

export const decodeAuthoritySet = (data: any) => {
  const justification = justificationDecode(data)

  return justification.commit.precommits.map((entry) => entry.id.toHex()).sort()
}

/**
 * Extract the authorities from the finality proof
 * @param finalityProof - The finality proof to extract the authorities from
 */

export const extractAuthoritySetFromFinalityProof = (finalityProof: any) => {
  const rawJust = finalityProofDecode(finalityProof).justification

  return decodeAuthoritySet(rawJust)
}

/**
 * Decode the header
 * @param data - The data to decode
 */

export const headerDecode = (data: string) => {
  registry.register(header)

  return createType(registry, header.type as any, data)
}

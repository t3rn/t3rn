import BN from 'bn.js'
import { u8aConcat, u8aToU8a, u8aToHex } from '@polkadot/util'
import { xxhashAsU8a } from '@polkadot/util-crypto'

/**
 * Creates to correct parachainId encoding for storage proofs
 * @param id - The parachain id to encode
 */

export const encodeParachainId = (id: number) => {
  return '0x' + new BN(id).toBuffer('le', 4).toString('hex')
}

/**
 * Creates to correct parachainId encoding for storage proofs
 * @param module - The module name
 * @param variableName - The variable name
 * @param parachainId - The parachain id
 */

export const generateArgumentKey = (
  module: string,
  variableName: string,
  parachainId: number,
) => {
  // lets prepare the storage key for system events.
  let module_hash = xxhashAsU8a(module, 128)
  let storage_value_hash = xxhashAsU8a(variableName, 128)

  let encodedParachainId = encodeParachainId(parachainId)
  let argumentKey = u8aConcat(
    xxhashAsU8a(encodedParachainId, 64),
    u8aToU8a(encodedParachainId),
  )

  // Special syntax to concatenate Uint8Array
  let final_key = new Uint8Array([
    ...module_hash,
    ...storage_value_hash,
    ...argumentKey,
  ])

  return u8aToHex(final_key)
}

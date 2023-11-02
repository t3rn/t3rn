import { decodeAddress, encodeAddress } from '@polkadot/keyring'
const { hexToU8a, isHex } = require('@polkadot/util')

/**
 * Convert a substrate pubkey to address
 * @param pubkey - The public key to convert
 * @param prefix - The ss58 format to use
 */

export const pub2Addr = (pub: string, prefix: number): string | Error => {
  if (isValidAddressPolkadotAddress(pub)) {
    pub = pub.split('0x')[1]
    return encodeAddress(Uint8Array.from(Buffer.from(pub, 'hex')), prefix)
  } else {
    return new Error('Invalid Address')
  }
}

/**
 * Convert a substrate address to public key. Passing public key will simply return it
 * @param address - The address to convert
 */

export const addrToPub = (address: string): string => {
  if (isValidAddressPolkadotAddress(address)) {
    if (isHex(address)) {
      // if hex, a pub was passed
      return address
    } else {
      return '0x' + Buffer.from(decodeAddress(address)).toString('hex')
    }
  } else {
    throw new Error('Invalid Address Detected!')
  }
}

/**
 * Check if a substrate pub or address is valid
 * @param address - The address to convert
 */

export const isValidAddressPolkadotAddress = (address: string): boolean => {
  try {
    encodeAddress(isHex(address) ? hexToU8a(address) : decodeAddress(address))
    return true
  } catch (error) {
    return false
  }
}

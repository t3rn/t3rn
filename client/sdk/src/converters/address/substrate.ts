import { decodeAddress, encodeAddress } from "@polkadot/keyring";
const { hexToU8a, isHex } = require('@polkadot/util');

// convert pubkey to address
export const pub2Addr = (pub: string, prefix: number): string | Error => {
	if(isValidAddressPolkadotAddress(pub)) {
		pub = pub.split('0x')[1]
		return encodeAddress(Uint8Array.from(Buffer.from(pub, 'hex')), prefix)
	} else {
		return new Error("Invalid Address")
	}
}

// convert address to public key
export const addrToPub = (address: string): string => {
	if(isValidAddressPolkadotAddress(address)) {
		if(isHex(address)) { // if hex odds are a pub was passed
			return address
		} else {
			return "0x" + Buffer.from(decodeAddress(address)).toString('hex')
		}
	} else {
		throw new Error("Invalid Address Detected!")
	}
}

// check if pub or addr are valid
export const isValidAddressPolkadotAddress = (address: string): boolean => {
	try {
		encodeAddress(
		isHex(address)
				? hexToU8a(address)
				: decodeAddress(address)
		);
		return true;
	} catch (error) {
		return false;
	}
};



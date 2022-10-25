const { u8aToHex, isHex, isString, isNumber } = require('@polkadot/util');
import * as BN from 'bn.js'

// A amount conversion class that can be used to convert amounts to the correct format based on the gateway
export class AmountConverter {
	value: BN | number;
	decimals: number;
	valueTypeSize: number;

	// Parameters:
	// value: two types are possible with different encodings
	// 		- Integer 		-> BN, string or number
	// 		- LittleEndian	-> hex string
	// decimals: the decimals of the target -> Default: 12
	// valueTypeSize: value type size in bytes -> Default: 16 (u128)
	constructor(args: {
		value?: number | BN | string;
		decimals?: number;
		valueTypeSize?: number;
	}) {
		// Set defaults
		if(!args.decimals) args.decimals = 12;
		if(!args.valueTypeSize) args.valueTypeSize = 16;

		this.decimals = args.decimals;
		this.valueTypeSize = args.valueTypeSize;

		if(args.value) {
			// interpret string values correctly
			if(isString(args.value)) {
				if(isHex(args.value)) {
					this.value = fromLeEncoding(args.value as string)
				} else {
					this.value = new BN.BN(args.value, 10)
				}
			} else {
				if(isNumber(args.value) && Math.floor(args.value as number) !== args.value) {
					throw new Error("AmountConverter: Float values not supported! Please convert to Integer!")
				}
				this.value = args.value as BN | number;
			}
		}
	}

	// convert the initialzed value into a float
	toFloat(): number {
		return bnToFloat(this.toBn(), this.decimals)
	}

	// convert the initialized value into BN
	toBn(): BN {
		return this.value as BN
	}

	// convert a float parameter into BN
	floatToBn(value: number): BN {
		return floatToBn(value, this.decimals)
	}

	// convert the initialized value into a LittleEndian byte array
	toLeArray(): number[] {
		return toLeEncoding(this.value as BN, this.valueTypeSize)
	}

	// convert the initialized value into LittleEndian encoded hex string
	toLeHex(): string {
		return u8aToHex(this.toLeArray())
	}
}

// converts hex LittleEndian to BN
export const fromLeEncoding = (number: string): BN => {
	return new BN.BN(number.split("0x")[1], 16,"le")
}

// encode number to LE array
export const toLeEncoding = (number: BN, valueTypeSize: number | undefined): number[] => {
	if(valueTypeSize === undefined) {
		valueTypeSize = 16 as number;
	}
	return new BN.BN(number).toArray("le", valueTypeSize)
}

// converts decimal number to uint as BigNumber. BN.js takes care of correct rounding here
export const floatToBn = (number: number, decimals: number): BN => {
	return new BN.BN(number * Math.pow(10, decimals))
}

// converts uint to a human readable decimal
export const bnToFloat = (number: BN, decimals: number): number => {
	return number.toNumber() / Math.pow(10, decimals)
}

// generate the optional insurance construct.
// As the resulting value is only used on t3rn, we rely on the correct default values here
export const optionalInsurance = (insurance: number | BN | string, reward: number | BN | string) =>  {
	const encodedInsurance = new AmountConverter({value: insurance}).toLeArray()
	const encodedReward = new AmountConverter({value: reward}).toLeArray()
	return u8aToHex([...encodedInsurance, ...encodedReward])
}


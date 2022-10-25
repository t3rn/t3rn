const { u8aToHex, isHex, isString, isNumber } = require('@polkadot/util');
import * as BN from 'bn.js'

export class AmountConverter {
	value: BN | number;
	decimals: number;
	valueTypeSize: number;

	constructor(args: {
		value?: number | BN | string;
		decimals?: number | undefined;
		valueTypeSize?: number | undefined;
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

		console.log("value", this.value)
	}

	toFloat(): number {
		return bnToFloat(this.toInt(), this.decimals)
	}

	toInt(): BN {
		return this.value as BN
	}

	floatToBn(value: number): BN {
		return floatToBn(value, this.decimals)
	}

	toLeArray(): number[] {
		return toLeEncoding(this.value as BN, this.valueTypeSize)
	}

	toLeHex() {
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
	console.log("Insurance:", insurance)

	const encodedInsurance = new AmountConverter({value: insurance}).toLeArray()
	console.log(encodedInsurance)
	const encodedReward = new AmountConverter({value: reward}).toLeArray()
	return u8aToHex([...encodedInsurance, ...encodedReward])
}


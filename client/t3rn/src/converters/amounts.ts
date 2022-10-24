import { BN } from "bn.js"
const { u8aToHex } = require('@polkadot/util');

export enum AmountType {
	Integer,
	Float,
	LittleEndian
	// BigNumber,
}

export class Amount {
	type: AmountType;
	value: number | string
	decimals: number;
	valueTypeSize: number;

	constructor(args: {
		type: AmountType;
		value: number | string;
		decimals?: number | undefined;
		valueTypeSize?: number | undefined;
	}) {
		// type specific constraint checks
		if(args.type === AmountType.LittleEndian) {
			if(typeof args.value !== 'string') throw new Error("Little Endian must be passed in hex!")
		}

		if(args.type === AmountType.Float) {
			if(typeof args.value !== 'number') throw new Error("Floats must be passed as numbers!")
			if(Number.isSafeInteger((args.value))) throw new Error("Must be passed as float!")
		}

		if(args.type === AmountType.Integer) {
			if(typeof args.value !== 'number') throw new Error("Floats must be passed as numbers!")
			if(!Number.isSafeInteger((args.value))) throw new Error("Must be passed as Integer!")
		}

		// Set defaults
		if(!args.decimals) args.decimals = 12;
		if(!args.valueTypeSize) args.valueTypeSize = 16;

		this.type = args.type;
		this.value = args.value;
		this.decimals = args.decimals;
		this.valueTypeSize = args.valueTypeSize;
	}

	toFloat(): number {
		switch(this.type) {
			case AmountType.Float:
				return this.value as number;
				break;
			case AmountType.Integer:
				return (this.value as number / Math.pow(10, this.decimals));
				break;
			case AmountType.LittleEndian:
				return intToFloat(this.toInt(), this.decimals)

		}
	}

	toInt(): number {
		switch(this.type) {
			case AmountType.Float:
				return floatToInt(this.value as number, this.decimals)
				break;
			case AmountType.Integer:
				return this.value as number
				break;
			case AmountType.LittleEndian:
				return fromLeEncoding(this.value as string).toNumber()

		}
	}

	toLeArray() {
		switch(this.type) {
			case AmountType.Float:
				return toLeEncoding(this.toInt() as number, this.valueTypeSize)
				break;
			case AmountType.Integer:
				return toLeEncoding(this.value as number, this.valueTypeSize)
				break;
			case AmountType.LittleEndian:
				return this.value as string
				break;
		}
	}

	toLeHex() {
		return u8aToHex(this.toLeArray())
	}

}

// converts LE encoded number to BN
// ToDo: fix BN return type
export const fromLeEncoding = (number: string): any => {
	new BN(number.split("0x")[1], 16,"le")
}

// encode number to LE array
export const toLeEncoding = (number: number, valueTypeSize: number | undefined): number[] => {
	if(valueTypeSize === undefined) {
		valueTypeSize = 32 as number;
	}
	return new BN(number).toArray("le", valueTypeSize)
}

// converts decimal number to uint as BigNumber. BN.js takes care of correct rounding here
export const floatToInt = (number: number, decimals: number): any => {
	return new BN(number * Math.pow(10, decimals))
}

// converts uint to a human readable decimal
export const intToFloat = (number: number, decimals: number): number => {
	return number / Math.pow(10, decimals)
}

export const optionalInsurance = (insurance: number, reward: number) =>  {
	const encodedInsurance = new Amount({value: insurance, type: 0}).toLeArray()
	const encodedReward = new Amount({value: reward, type: 0}).toLeArray()
	return u8aToHex([...encodedInsurance, ...encodedReward])
}


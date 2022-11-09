import {ApiPromise} from "@polkadot/api";
import BN from "bn.js";
import {AmountConverter} from "../converters/amounts";
import {Tx} from "./tx";

const DECIMALS = 12;
const VALUE_TYPE_SIZE = 16;

export class Circuit {
	api: ApiPromise;
	// TODO get correct type
	signer: any;
	tx: Tx;

	constructor(api: ApiPromise, signer: any) {
		this.api = api
		this.signer = signer
		this.tx = new Tx(this.api, this.signer)
	}

	// converts a float to a BN with the correct decimal precision
	floatToBn(value: number): BN {
		return new AmountConverter({
			decimals: DECIMALS,
			valueTypeSize: VALUE_TYPE_SIZE}
		).floatToBn(value)
	}

	toFloat(value: BN | number): number {
		return new AmountConverter({
			value,
			decimals: DECIMALS,
			valueTypeSize: VALUE_TYPE_SIZE}
		).toFloat()
	}
}

// @ts-ignore
import {T3rnPrimitivesXdnsXdnsRecord, T3rnTypesSideEffect} from "@polkadot/types/lookup"
import {AmountConverter, optionalInsurance} from "../converters/amounts";
import * as address from "../converters/address";
import * as BN from 'bn.js'
import { toU8aId } from "../converters/utils";
import { createSfx } from "../side-effects";

export enum GatewayType {
	Substrate,
	Evm
}

export class Gateway {
	id: string;
	rpc: string;
	vendor: string;
	type: GatewayType;
	ticker: string;
	decimals: number;
	addressFormat: number;
	valueTypeSize: number;
	allowedSideEffects: string[];
	createSfx: {} = {};

	constructor(xdnsEntry: T3rnPrimitivesXdnsXdnsRecord) {
		this.id = xdnsEntry.toHuman().gateway_id;
		this.rpc = xdnsEntry.url.toHuman();
		// @ts-ignore
		this.vendor = xdnsEntry.toHuman().gateway_vendor.toString();
		this.type = this.getType(xdnsEntry.toHuman().gateway_vendor.toString());
		// @ts-ignore
		this.ticker = xdnsEntry.toHuman().gateway_sys_props.token_symbol;
		// @ts-ignore
		this.decimals = parseInt(xdnsEntry.toHuman().gateway_sys_props.token_decimals);
		// @ts-ignore
		this.addressFormat = parseInt(xdnsEntry.toHuman().gateway_sys_props.ss58_format);
		// @ts-ignore
		this.valueTypeSize = parseInt(xdnsEntry.toHuman().gateway_abi.value_type_size);
		this.allowedSideEffects = xdnsEntry.toHuman().allowed_side_effects
		this.setSfxBindings();
	}

	createTransferSfx = (
		args: {
			from: string,
			to: string,
			value: number | BN | string,
			maxReward: number | BN | string,
			insurance: number | BN | string,
			nonce: number,
			signature?: string,
			enforceExecutioner?: string
		}
	): T3rnTypesSideEffect => {
		const encodedArgs: string[] = this.encodeTransferArgs(args.from, args.to, args.value, args.insurance, args.maxReward)

		const maxReward = new AmountConverter({value: args.maxReward}).toBn()
		const insurance = new AmountConverter({value: args.insurance}).toBn()

		return createSfx({
			target: toU8aId(this.id),
			nonce: args.nonce,
			maxReward,
			insurance,
			encodedArgs,
			encodedAction: "tran",
			signature: args.signature,
			enforceExecutioner: args.enforceExecutioner,
		})
	}

	encodeTransferArgs(
		from: string,
		to: string,
		value: number | BN | string,
		insurance: number | BN | string,
		reward: number | BN | string,
	): string[] {
		if(!this.allowedSideEffects.includes("tran")) throw new Error(`Transfer Sfx not supported for ${this.id}`)
		// ensure we pass the correct address encoding (e.g. pub key for substrate)
		from = this.validateAddress(from)
		to = this.validateAddress(to)

		// convert value to LittleEndian
		const encodedAmount = new AmountConverter({
			value,
			decimals: this.decimals,
			valueTypeSize: this.valueTypeSize}
		).toLeHex()

		// generate optionalInsurance
		const encodedOptionalInsurance = optionalInsurance(insurance, reward)
		return [from, to, encodedAmount, encodedOptionalInsurance]
	}

	// Convert an address into t3rn compatible form. For example, we want to ensure we pass the public key for polkadot addresses
	validateAddress(addr: string) {
		switch(this.type) {
			case GatewayType.Substrate:
				return address.substrate.addrToPub(addr)
				break;
			default:
				return addr;
		}
	}

	// convert a float value into the correct integer, accounting for decimals
	floatToBn(value: number): BN {
		return new AmountConverter({
			decimals: this.decimals,
			valueTypeSize: this.valueTypeSize}
		).floatToBn(value)
	}

	getType(vendor: string) {
		if(vendor === "Rococo" || vendor === 'Kusama' || vendor === 'Polkadot') {
			return GatewayType.Substrate
		} else if(vendor === "Ethereum") {
			return GatewayType.Evm
		}
	}

	setSfxBindings() {
		for(let i = 0; i < this.allowedSideEffects.length; i++) {
			switch(this.allowedSideEffects[i]) {
				case "tran":
					this.createSfx["tran"] = this.createTransferSfx

			}
		}
	}


}
import "@t3rn/types"
// @ts-ignore
import {T3rnTypesSideEffect} from "@polkadot/types/lookup";

import {toId, toIdString} from "../converters/utils";
import {addrToPub} from "../converters/address/substrate";
import {AmountType, Amount, optionalInsurance} from "../converters/amounts";
import {Converter} from "../converters";

export class Transfer {
	converter: Converter;

	constructor(converter: Converter) {
		this.converter = converter
	}

	create(
		args: {
			target: string | number[],
			signature: string | undefined,
			nonce: number,
			enforceExecutioner: string | undefined,
			from: string,
			to: string,
			maxReward: number,
			insurance: number,
			amount: number | string,
			amountType?: AmountType // describe which type should be used for the values (amount, reward, insurance). DEFAULT: Int
		}
	) {
		// set defaults
		if(!args.amountType) args.amountType = AmountType.Integer;

		// the defaults are set to t3rns values, so we dont need to pass the decimals
		args.maxReward = new Amount({value: args.maxReward, type: args.amountType}).toInt()
		args.insurance = new Amount({value: args.insurance, type: args.amountType}).toInt()

		const sfx: T3rnTypesSideEffect = {
			target: toId(args.target),
			maxFee: args.maxReward,
			insurance: args.insurance,
			encodedAction: toId("tran"),
			encodedArgs: this.encodeArgs(args.from, args.to, args.amount, args.insurance, args.maxReward, toIdString(args.target), args.amountType),
			signature: args.signature,
			requesterNonce: args.nonce,
			enforceExecutioner: (args.enforceExecutioner ? addrToPub(args.enforceExecutioner) : args.enforceExecutioner),
		}

		console.log(sfx)
	}

	encodeArgs(from: string, to: string, amount: number | string, insurance: number, reward: number, target: string, amountType: AmountType): string[] {

		from = this.converter.validateTargetAddress(from, toIdString(target))
		to = this.converter.validateTargetAddress(to, toIdString(target))
		// convert to target amounts
		const {decimalsTarget} = this.converter.xdns.gateways[toIdString(target)];
		const encodedAmount = new Amount({value: amount, decimals: decimalsTarget, type: amountType}).toLeHex()
		const encodedOptionalInsurance = optionalInsurance(insurance, reward)

		return [from, to, encodedAmount, encodedOptionalInsurance]
	}
}

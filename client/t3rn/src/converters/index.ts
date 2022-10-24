import * as address from "./address";
import * as utils from "./utils"
import {GatewayType, XDNS} from "../xdns";
import {Amount} from './amounts'
import * as amount from "./amounts";

export class Converter {
	xdns: XDNS;

	constructor(xdns: XDNS) {
		this.xdns = xdns;
	}

	decimalToTargetInt(value: number, target: string) {
		const {decimals, valueTypeSize} = this.xdns.gateways[target];
		return new Amount({value, decimals, valueTypeSize, type: 1})
	}

	validateTargetAddress(addr: string, target: string) {
		const { gatewayType } = this.xdns.gateways[target];
		switch(gatewayType) {
			case GatewayType.Substrate:
				return address.substrate.addrToPub(addr)
				break;
			default:
				return addr;

		}
	}
}


export {address, amount, utils}
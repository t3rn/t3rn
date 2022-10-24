import { ApiPromise } from "@polkadot/api";
// @ts-ignore
import {T3rnPrimitivesXdnsXdnsRecord} from "@polkadot/types/lookup"

export enum GatewayType {
	Substrate,
	Evm
}

export type Gateway = {
	id: string,
	rpc: string,
	vendor: string,
	type: GatewayType,
	ticker: string,
	decimals: number,
	addressFormat: number,
	valueTypeSize: number,
}

export class XDNS {
	api: ApiPromise;
	records: T3rnPrimitivesXdnsXdnsRecord[];
	gateways: Gateway[] = [];
	idMapper: {
		[id: string]: number
	} = {};

	constructor(api: ApiPromise) {
		this.api = api
	}

	async setup() {
		return new Promise(async (res, rej) =>{
			// @ts-ignore
			const records: T3rnPrimitivesXdnsXdnsRecord[] = await this.api.rpc.xdns.fetchRecords();
			this.records = records['xdns_records']; // the weirdness of TS

			for (let j = 0; j < this.records.length; j++) {
				const gateway: Gateway = {
					id: this.records[j].toHuman().gateway_id,
					rpc: this.records[j].url.toHuman(),
					// @ts-ignore
					vendor: this.records[j].toHuman().gateway_vendor.toString(),
					type: this.getType(this.records[j].toHuman().gateway_vendor.toString()),
					// @ts-ignore
					ticker: this.records[j].toHuman().gateway_sys_props.token_symbol,
					// @ts-ignore
					decimals: parseInt(this.records[j].toHuman().gateway_sys_props.token_decimals),
					// @ts-ignore
					addressFormat: parseInt(this.records[j].toHuman().gateway_sys_props.ss58_format),
					// @ts-ignore
					valueTypeSize: parseInt(this.records[j].toHuman().gateway_abi.value_type_size),
				}
				this.idMapper[gateway.id] = this.gateways.length;
				this.gateways.push(gateway)
			}
			res(true)
		})
	}

	getType(vendor: string) {
		if(vendor === "Rococo" || vendor === 'Kusama' || vendor === 'Polkadot') {
			return GatewayType.Substrate
		} else if(vendor === "Ethereum") {
			return GatewayType.Evm
		}
	}
}
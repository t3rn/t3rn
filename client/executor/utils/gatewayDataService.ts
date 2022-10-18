import {ApiPromise, WsProvider} from "@polkadot/api";
import {T3rnPrimitivesXdnsXdnsRecord} from "@polkadot/types/lookup"
import createDebug from "debug"

const BN = require("bn.js");

type Gateway = {
	id: string,
	name: string,
	rpc: string,
	vendor: string,
	type: string,
	ticker: string,
	decimals: number,
	addressFormat: number,
	rpcClient: ApiPromise,
	signer: string | undefined,
}

export class GatewayDataService {
	config: any;
	xdnsRecords: T3rnPrimitivesXdnsXdnsRecord[]
	gateways: Gateway[] = [];
	idMapper: {
		[id: string]: number
	} = {};

	circuitClient: ApiPromise;

	static debug = createDebug("gateway-service")

	constructor(circuitClient: ApiPromise, config: any) {
		this.config = config;
		this.circuitClient = circuitClient;
	}

	async init() {
		return new Promise(async (res, _) => {
			// @ts-ignore
			const records: T3rnPrimitivesXdnsXdnsRecord[] = await this.circuitClient.rpc.xdns.fetchRecords();
			this.xdnsRecords = records['xdns_records']; // the weirdness of TS

			for (let i = 0; i < this.config.gateways.length; i++) {
				for (let j = 0; j < this.xdnsRecords.length; j++) {
					if (this.config.gateways[i].id === this.xdnsRecords[j].toHuman().gateway_id) {

						const rpcClient = await ApiPromise.create({
							provider: new WsProvider(this.config.gateways[i].rpc),
						});

						const gateway: Gateway = {
							id: this.config.gateways[i].id,
							name: this.config.gateways[i].name,
							rpc: this.config.gateways[i].rpc,
							// @ts-ignore
							vendor: this.xdnsRecords[j].toHuman().gateway_vendor.toString(),
							type: this.config.gateways[i].type,
							// @ts-ignore
							ticker: this.xdnsRecords[j].toHuman().gateway_sys_props.token_symbol,
							// @ts-ignore
							decimals: parseInt(this.xdnsRecords[j].toHuman().gateway_sys_props.token_decimals),
							// @ts-ignore
							addressFormat: parseInt(this.xdnsRecords[j].toHuman().gateway_sys_props.ss58_format),
							rpcClient,
							signer: this.config.gateways[i].signer
						}
						this.idMapper[gateway.id] = this.gateways.length;
						this.gateways.push(gateway)
						break;
					}
				}
			}
			return res(true)
		})
	}

	// creates human readable value format
	valueToHuman(amount: number, id: string) {
		const decimals = this.gateways[this.idMapper[id]].decimals;
		const num = amount / Math.pow(10, decimals);
		console.log(num)
		return num;
	}

	// creates uint value format
	humanToValue(amount: number, id: string) {
		const decimals = this.gateways[this.idMapper[id]].decimals;
		const num = new BN(amount * Math.pow(10, decimals));
		console.log(num)
		return num;
	}
}




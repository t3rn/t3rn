import cost from "../../config/cost.json"
import { ApiPromise } from "@polkadot/api"
import {SideEffect} from "../../circuit/executions/sideEffect";

export default class CostEstimator {
	baseFee: number;
	weights: {
		[sfxId: string]: number,
	}
	lengthFee: number;
	tip: number;
	chainId: string;
	signer: any;
	client: ApiPromise;

	constructor(chainId: string, client: ApiPromise, signer: any) {
		this.chainId = chainId;
		this.client = client
		this.setup();
		this.signer = signer;
		// this.fetchFeeDetails()
	}

	async setup() {
		console.log(await this.fetchTxFees("0xe91c2593ccb56a7f2e8cf72b1dd0f984d5383f09d912808fa312a5e9b603cca9", 2))
	}

	// fetches tx fee details from endpoint. Not used to estimate fees, but might be useful again
	async fetchTxFees(blockHash: string, extrinsicIndex:number) {
		const { block } = await this.client.rpc.chain.getBlock(blockHash);

		const queryFeeDetails = await this.client.rpc.payment.queryFeeDetails(block.extrinsics[extrinsicIndex].toHex(), blockHash);
		const queryInfo = await this.client.rpc.payment.queryInfo(block.extrinsics[extrinsicIndex].toHex(), blockHash);

		// @ts-ignore
		const baseFee = queryFeeDetails.inclusionFee.toJSON().baseFee; // the base fee in native asset of an extrinsic. I think this should be be the same for all extrinsics
		// @ts-ignore
		const lengthFee = queryFeeDetails.inclusionFee.toJSON().lenFee; // fee based on the byte length of a transaction in native asset
		// @ts-ignore
		const adjustedWeightFee =  queryFeeDetails.inclusionFee.toJSON().adjustedWeightFee; // fee based on extrinsic weight in native asset


		const partialFee = queryInfo.partialFee.toNumber(); // tthe total tx fee in the native asset, without tips
		const txWeight = queryInfo.weight.toNumber(); // weight of the extrinsic

		return {baseFee, lengthFee, adjustedWeightFee, txWeight, partialFee}
	}
}


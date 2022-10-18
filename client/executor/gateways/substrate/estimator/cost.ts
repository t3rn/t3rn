import {SubmittableExtrinsic} from "@polkadot/api/promise/types";
import SubstrateRelayer from "../relayer";

export default class CostEstimator {
	relayer: SubstrateRelayer;

	constructor(relayer: SubstrateRelayer) {
		this.relayer = relayer
	}

	/// returns the transaction cost of a specific side effect in native asset
	async currentTransactionCost(tx: SubmittableExtrinsic) {
		const paymentInfo = await tx.paymentInfo(this.relayer.signer);
		return paymentInfo.partialFee.toJSON()
	}



	// // fetches tx fee details from endpoint. Not used to estimate fees, but might be useful again
	// async fetchTxFees(blockHash: string, extrinsicIndex:number) {
	// 	const { block } = await this.relayer.client.rpc.chain.getBlock(blockHash);
	//
	// 	const queryFeeDetails = await this.client.rpc.payment.queryFeeDetails(block.extrinsics[extrinsicIndex].toHex(), blockHash);
	// 	const queryInfo = await this.client.rpc.payment.queryInfo(block.extrinsics[extrinsicIndex].toHex(), blockHash);
	//
	// 	// @ts-ignore
	// 	const baseFee = queryFeeDetails.inclusionFee.toJSON().baseFee; // the base fee in native asset of an extrinsic. I think this should be be the same for all extrinsics
	// 	// @ts-ignore
	// 	const lengthFee = queryFeeDetails.inclusionFee.toJSON().lenFee; // fee based on the byte length of a transaction in native asset
	// 	// @ts-ignore
	// 	const adjustedWeightFee =  queryFeeDetails.inclusionFee.toJSON().adjustedWeightFee; // fee based on extrinsic weight in native asset
	//
	//
	// 	const partialFee = queryInfo.partialFee.toNumber(); // tthe total tx fee in the native asset, without tips
	// 	const txWeight = queryInfo.weight.toNumber(); // weight of the extrinsic
	//
	// 	return {baseFee, lengthFee, adjustedWeightFee, txWeight, partialFee}
	// }
}


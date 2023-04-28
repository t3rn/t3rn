import {Connection} from "./connection";
import {ApiPromise, Encodings} from "@t3rn/sdk";
const axios = require('axios').default;

export const generateRange = async (config: any, circuitConnection: Connection, targetConnection: Connection): Promise<any[]> => {
	return new Promise(async (resolve, reject) => {
		try {
			const circuitHeight = await currentGatewayHeight(circuitConnection, config.targetGatewayId)
			const targetHeight = await currentTargetHeight(targetConnection)

			if(targetHeight > circuitHeight) {
				let batches = await generateBatchProof(circuitConnection.client, targetConnection.client, config.targetGatewayId, circuitHeight + 1, targetHeight)
				resolve(batches)
			} else {
				console.log("No new blocks to submit")
				resolve([])
			}
		} catch(error) {
			reject(error)
		}
	})
}

const generateBatchProof = async (circuitClient: ApiPromise, targetClient: ApiPromise, targetGatewayId: string, from: number, to: number): Promise<any[]> => {

	let transactionArguments: any[] = [];
	while(from < to) {
		// get finalityProof element of epoch that contains block #from
		const finalityProof = await targetClient.rpc.grandpa.proveFinality(from)
		// decode finality proof
		let { justification, headers } = Encodings.Substrate.Decoders.finalityProofDecode(finalityProof)
		let signed_header = headers.pop()

		// query from header again, as its not part of the proof, and concat
		headers = [await getHeader(targetClient, from), ...headers]
		let range = circuitClient.createType("Vec<Header>", headers)

		justification = Encodings.Substrate.Decoders.justificationDecode(justification);

		//push to transaction queue
		transactionArguments.push({gatewayId: circuitClient.createType("ChainId", targetGatewayId), signed_header, range, justification})
		from = parseInt(signed_header.number.toJSON()) + 1
	}
	return transactionArguments;

}

const currentTargetHeight = async (connection: Connection): Promise<number> => {
	const header = await connection.client.rpc.chain.getHeader(
		await connection.client.rpc.chain.getFinalizedHead()
	);
	return header.number.toNumber();
}

const currentGatewayHeight = async (client: Connection, targetGatewayId: string)=> {
	return axios.post('http://localhost:9933', {
		jsonrpc: '2.0',
		method: 'portal_fetchHeadHeight',
		params: [Array.from(new TextEncoder().encode(targetGatewayId))],
		id: 1
	}, {
		headers: {
		'Content-Type': 'application/json'
		}
	})
	.then(response => {
	  	return response.data.result;
	})
}

const getHeader = async (client: ApiPromise, height: number) => {
    return (await client.rpc.chain.getHeader(
        await client.rpc.chain.getBlockHash(height)
    )).toJSON()
}
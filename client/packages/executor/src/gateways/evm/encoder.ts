import RLP from 'rlp';
import { ApiPromise } from "@polkadot/api"
import types from "./circuitTypes.json"

export const scaleEncodeProof = async (proof: any, index: number) => {
    const api = await ApiPromise.create({ types: types as any });

    let encoded = api.createType("Proof", {
        bytes: proof.map((entry: any) => "0x" + entry.toString('hex')),
        index: [...encode(index)]
    });

    return encoded.toHex()
}

export const encode = (input: any) => (input === '0x0')
    ? RLP.encode(Buffer.alloc(0))
    : RLP.encode(input);


export const receiptToRlp = (receipt: any, instance: any) => {
    const forEncoding = [
        receipt.status ? "0x1" : "0x",
        receipt.cumulativeGasUsed > 0 ? instance.utils.toHex(receipt.cumulativeGasUsed) : "0x",
        receipt.logsBloom,
        receipt.logs.map((log: any) => [log.address, log.topics, log.data])
    ];
    return RLP.encode(forEncoding);
};

export const scaleEncodeReceipt = async (txId: string, instance: any) => {
    const api = await ApiPromise.create({ types });
    let receipt = await instance.eth.getTransactionReceipt(txId)
    let topics = api.createType('Topics', [...receipt.logs[0].topics])

    const receiptObj = {
        status: receipt.status,
        cumulativeGasUsed: receipt.cumulativeGasUsed,
        logsBloom: receipt.logsBloom,
        logs: [{
            address: receipt.logs[0].address,
            topics,
            data: receipt.logs[0].data,
        }]
    }
    let encoded = api.createType('Receipt', receiptObj)
    console.log("SCALE RECEIPT:", encoded.toHex())

    return encoded.toHex()
}

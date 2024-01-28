const utils = require('web3-utils');
import RLP from 'rlp'
var Hash = require("eth-lib/lib/hash");

export const scaleEncodeHeader = async (rpcBlock: any, circuit: any) => {
    let poaBlock = rpcBlock;
    poaBlock['chainId'] = 56;
    delete poaBlock.hash;
    delete poaBlock.size;

    poaBlock = formatExtraData(poaBlock);

    let header = await circuit.createType('BscHeader', poaBlock)
    return header.toHex()
}

const formatExtraData = (header: any) => {
    const extraData = header.extraData.toLowerCase().split("0x")[1]
    header.extra = "0x" + extraData.slice(0, 64);
    if (header.number % 200 === 0) {
        header.validators = "0x" + extraData.slice(64, 904); // 20 byte * 21 + 64
        header.signature = "0x" + extraData.slice(904, 1034)
    } else {
        header.validators = null;
        header.signature = "0x" + extraData.slice(64, 194)
    }
    return header
}

export const scaleEncodeHash = async (hash: any, circuit: any) => {
    const encoded = await circuit.createType('H256', hash);
    console.log(encoded.toHex())
    return encoded
}

export const scaleEncodeHeaderRange = async (range: any, circuit: any) => {
    let res: any[] = [];
    for(let i = 0; i < range.length; i++) {
        const encoded = await scaleEncodeHeader(range[i], circuit)
        res.push(encoded)
    }
    return res
}

// this is the actual header hash that was signed
export const byteEncodeHeader = (header: any) => {
    return [
        utils.toHex(56), // chainID
        header.parentHash,
        header.sha3Uncles,
        header.miner,
        header.stateRoot,
        header.transactionsRoot,
        header.receiptsRoot,
        header.logsBloom,
        utils.toHex(header.difficulty),
        utils.toHex(header.number),
        utils.toHex(header.gasLimit),
        utils.toHex(header.gasUsed),
        utils.toHex(header.timestamp),
        processExtraData(header.extraData)[0], // => removes last 65 bytes
        header.mixHash,
        header.nonce
    ]
}

export const byteEncodeLookup = ((header: any) => {
    return [
        header.parentHash,
        header.sha3Uncles,
        header.miner,
        header.stateRoot,
        header.transactionsRoot,
        header.receiptsRoot,
        header.logsBloom,
        utils.toHex(header.difficulty),
        utils.toHex(header.number),
        utils.toHex(header.gasLimit),
        utils.toHex(header.gasUsed),
        utils.toHex(header.timestamp),
        header.extraData, // => removes last 65 bytes
        header.mixHash,
        header.nonce
    ]
})

export const hash = (encodedMsg: any) => {
    return Hash.keccak256(encodedMsg);
}

export const computeHash = (header: any, circuit: any) => {
    const byteEncoded = byteEncodeHeader(header);
    const encoded = RLP.encode(byteEncoded)
    const blockHash = hash("0x" + Buffer.from(encoded).toString('hex'))
    return blockHash
}

export const computeLookupHash = (header: any, circuit: any) => {
    const byteEncoded = byteEncodeLookup(header);
    const encoded = RLP.encode(byteEncoded)
    const blockHash = hash("0x" + Buffer.from(encoded).toString('hex'))
    return blockHash
}

export const processExtraData = (data: string) => {
    const index = data.length - 130; // 130 chars (65 Bytes) is the appended signature
    const extra = data.toLowerCase().slice(0, index);
    const sig = data.toLowerCase().slice(index)
    // console.log("signature:", sig)
    return [extra, sig]
}
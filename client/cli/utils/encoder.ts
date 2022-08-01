const BN = require('bn.js');

export const transferAmount = (amount: number, decimals: number, size: number) => {
    let bn = new BN(amount * Math.pow(10, decimals))
    return bn.toArray("le", size)
}

// encodes data for exporting. We export in encoded and human format.
// Encoded: We use for seeding protal rust tests
// Human: Debugging those tests and viewing data
export const encodeExport = (data: any, transactionType: string) => {
    if(Array.isArray(data)) {
        return data.map(entry => iterateEncode(entry, transactionType))
    } else {
        return iterateEncode(data, transactionType)
    }
}

const iterateEncode = (data: any, transactionType: string) => {
    let keys = Object.keys(data);
    let result = {};
    if(keys.includes("initialU8aLength")) { // this is a polkadot/apiPromise object
        return {
            data: data.toHuman(),
            transactionType,
            encoded_data: data.toHex().substring(2)
        }
    } else {
        for(let i = 0; i < keys.length; i++) {
            result['encoded_' + keys[i]] = data[keys[i]].toHex().substring(2)
            result[keys[i]] = data[keys[i]].toHuman()
        }
        result['transactionType'] = transactionType;
        return result;
    }
}
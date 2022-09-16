const BN = require('bn.js');

export const amountLeArr = (amount: number, decimals: any, size: any) => {
    let bn = computeDecimalsBN(amount, decimals)
    return bn.toArray("le", size)
}

export const computeDecimalsBN = (amount: number, decimals: any) => {
    return  new BN(amount * Math.pow(10, decimals))
}

export const optionalInsurance = (insurance: number, reward: number, decimals: number, size: number) =>  {
    const encodedInsurance = amountLeArr(insurance, decimals, size);
    const encodedReward = amountLeArr(reward, decimals, size);
    return [...encodedInsurance, ...encodedReward]
}

// encodes data for exporting. We export in encoded and human format.
// Encoded: We use for seeding protal rust tests
// Human: Debugging those tests and viewing data
export const encodeExport = (data: any, transactionType: string, submissionHeight: number) => {
    if(Array.isArray(data)) {
        return data.map(entry => iterateEncode(entry, transactionType, submissionHeight))
    } else {
        return iterateEncode(data, transactionType, submissionHeight)
    }
}

const iterateEncode = (data: any, transactionType: string, submissionHeight: number) => {
    let keys = Object.keys(data);
    let result = {};
    if(keys.includes("initialU8aLength")) { // this is a polkadot/apiPromise object
        return {
            data: data.toHuman(),
            transaction_type: transactionType,
            encoded_data: data.toHex().substring(2)
        }
    } else {
        for(let i = 0; i < keys.length; i++) {
            result['encoded_' + toSnakeCase(keys[i])] = data[keys[i]].toHex().substring(2)
            result[toSnakeCase(keys[i])] = data[keys[i]].toHuman()
        }
        result['transaction_type'] = transactionType;
        result['submission_height'] = submissionHeight
        return result;
    }
}

const toSnakeCase = str =>
  str &&
  str
    .match(/[A-Z]{2,}(?=[A-Z][a-z]+[0-9]*|\b)|[A-Z]?[a-z]+[0-9]*|[A-Z]|[0-9]+/g)
    .map(x => x.toLowerCase())
    .join('_');
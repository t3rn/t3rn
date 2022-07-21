const BN = require('bn.js');

export const transferAmount = (amount: number, decimals: number, size: number) => {
    let bn = new BN(amount * Math.pow(10, decimals))
    return bn.toArray("le", size)
}

// encodes data for exporting. We export in encoded and human format.
// Encoded: We use for seeding protal rust tests
// Human: Debugging those tests and viewing data
export const encodeExport = (data: any) => {
    let keys = Object.keys(data);
    for(let i = 0; i < keys.length; i++) {
        data['encoded_' + keys[i]] = data[keys[i]].toHex().substring(2)
        data[keys[i]] = data[keys[i]].toHuman()
    }
    return data;
}
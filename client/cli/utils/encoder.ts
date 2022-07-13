const BN = require('bn.js');

export const transferAmount = (amount: number, decimals: number, size: number) => {
    let bn = new BN(amount * Math.pow(10, decimals))
    return bn.toArray("le", size)
}
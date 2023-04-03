const solc = require('solc');
const fs = require("fs");

const contractSourceCode = fs.readFileSync('contract.sol', 'utf8');

const compilationInput = {
    language: 'Solidity',
    sources: {
        'MyContract.sol': {
            content: contractSourceCode
        }
    },
    settings: {
        outputSelection: {
            '*': {
                '*': [ 'abi', 'evm.bytecode' ]
            }
        }
    }
};

export const compile = () => {
    const compiledContract = solc.compile(JSON.stringify(compilationInput));
    const contractBytecode = JSON.parse(compiledContract).contracts['MyContract.sol']['Counter'].evm.bytecode.object;
    return contractBytecode;
    // const contractAbi = JSON.parse(compiledContract).contracts['MyContract.sol']['Counter'].abi;

}

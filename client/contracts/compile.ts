const solc = require('solc');
const fs = require("fs");

const contractSourceCode = fs.readFileSync('contract.sol', 'utf8');

const compilationInput = {
    language: 'Solidity',
    sources: {
        'contract.sol': {
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
    console.log(compiledContract);
    const contractBytecode = JSON.parse(compiledContract).contracts['contract.sol']['Snorkle'].evm.bytecode.object;
    return contractBytecode;
}

export const getAbi = () => {
    const compiledContract = solc.compile(JSON.stringify(compilationInput));
    const contractAbi = JSON.parse(compiledContract).contracts['contract.sol']['Snorkle'].abi;
    return contractAbi;
}

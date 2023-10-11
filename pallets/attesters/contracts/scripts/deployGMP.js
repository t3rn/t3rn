// scripts/deploy.js
const { ethers } = require('hardhat');

async function main() {
    const ContractVault = await ethers.getContractFactory('t3rnVault');
    const contractt3rnVault = await ContractVault.deploy()
    await contractt3rnVault.deployed();

    console.log('Contract t3rnVault deployed to address:', contractt3rnVault.address);


    const ContractEscrowGMP = await ethers.getContractFactory('EscrowGMP');
    const contractEscrowGMP = await ContractEscrowGMP.deploy(contractt3rnVault.address) // contractt3rnVault.address
    await contractEscrowGMP.deployed();

    console.log('Contract EscrowGMP deployed to address:', contractEscrowGMP.address);

    const ContractXOrder = await ethers.getContractFactory('RemoteOrder');
    const contractxOrder = await ContractXOrder.deploy(contractEscrowGMP.address, contractt3rnVault.address)
    await contractxOrder.deployed();

    console.log('Contract RemoteOrder deployed to address:', contractxOrder.address);

    const ContractLocalExchange = await ethers.getContractFactory('LocalExchange');
    const contractLocalExchange = await ContractLocalExchange.deploy()
    await contractLocalExchange.deployed();

    console.log('Contract LocalExchange deployed to address:', contractLocalExchange.address);

    const ContractAttesters = await ethers.getContractFactory('AttestationsVerifierProofs');
    const contractAttesters = await ContractAttesters.deploy([], [], 1, contractEscrowGMP.address);
    await contractAttesters.deployed();

    console.log('Contract AttestationsVerifierProofs deployed to address:', contractAttesters.address);
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });
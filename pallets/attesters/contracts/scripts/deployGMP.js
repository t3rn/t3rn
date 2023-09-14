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

    console.log('Contract escrowGMP deployed to address:', contractEscrowGMP.address);

    const ContractXOrder = await ethers.getContractFactory('RemoteOrder');
    const contractxOrder = await ContractXOrder.deploy(contractEscrowGMP.address)
    await contractxOrder.deployed();

    console.log('Contract contractxOrder deployed to address:', contractxOrder.address);

    contractxOrderRes = await contractxOrder.remoteOrder("0x03030303000000000000000000000000000000000000000000000000000000000000e80300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000640000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000064");

    console.log('Contract contractxOrderRes res address:', contractxOrder.address);

}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });
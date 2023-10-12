// scripts/deploy.js
const { ethers } = require('hardhat');

async function main() {

    const ContractGMP = await ethers.getContractFactory('EscrowGMP');
    const contractGMP = ContractGMP.attach("0x9f345c72Ae8BeFEA11d96FbF0D8920a2A71dF370");
    const storePayloadCall = await contractGMP.storeRemoteOrderPayload("0x0909090909090909090909090909090909090909090909090909090909090909", "0x0909090909090909090909090909090909090909090909090909090909090909");
    const storePayloadReceipt = await storePayloadCall.wait();
    console.log('Call ContractGMP.storeRemoteOrderPayload result:', storePayloadReceipt.cumulativeGasUsed.toString());
    const ContractXOrder = await ethers.getContractFactory('RemoteOrder');
    const contract = ContractXOrder.attach("0x4e8540F5a94CCcDFBd1a1290F68c1BFbD3EA04E3");
    const contractxOrderRes = await contract.remoteOrder("0x03030303050505050000000000000000000000000000000000000000000000000000000009090909090909090909090909090909090909090909090909090909090909090000000000000000000000000000000000000000000000000000000000000064000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000640000000000000000000000000000000000000000000000000000000000000064", { value: ethers.utils.parseUnits('100','wei') })
    const contractxOrderReceipt = await contractxOrderRes.wait();
    // const contractxOrderRes = await contract.remoteOrderDecoded("0x03030303", "0x05050505", "0x0909090909090909090909090909090909090909090909090909090909090909", 100, "0x0000000000000000000000000000000000000000", 100, 100, { value: ethers.utils.parseUnits('100','wei') });
    // const contractxOrderRes = await contract.generateId("0x1932978B1Ee86aD07E3e324ACc1641DBf14dbF59", "1")
    console.log('Call ContractXOrder.remoteOrder receipt:', contractxOrderReceipt);
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });
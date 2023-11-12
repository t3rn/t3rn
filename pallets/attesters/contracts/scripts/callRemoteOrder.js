// scripts/deploy.js
const { ethers } = require('hardhat');

async function main() {
    const ContractGMP = await ethers.getContractFactory('EscrowGMP');
    const contractGMP = ContractGMP.attach(
        '0x3f2a7B4101b49C40dF12b92e21eeF2D0a4a5B867'
    );
    const storePayloadCall = await contractGMP.storeRemoteOrderPayload(
        '0x0909090909090909090909090909090909090909090909090909090909090909',
        '0x0909090909090909090909090909090909090909090909090909090909090909'
    );
    const storePayloadReceipt = await storePayloadCall.wait();
    console.log(
        'Call ContractGMP.storeRemoteOrderPayload result:',
        storePayloadReceipt.cumulativeGasUsed.toString()
    );
    const ContractXOrder = await ethers.getContractFactory('XOrder');
    const contract = ContractXOrder.attach(
        '0x34705639686234A9EA46Aadb7ed3BAA0C5df9eF4'
    );
    const contractxOrderRes = await contract.remoteOrder(
        '0x0303030300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006f39c4f2ec2bcf1cfb2c7b74f0c81c075b684dea367314850bcc5dc03cfcd71b6700000000000000000000000000000000000000000000000000000000000000640000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000006e',
        { value: ethers.utils.parseUnits('110', 'wei') }
    );
    const contractxOrderReceipt = await contractxOrderRes.wait();
    console.log(
        'Call ContractXOrder.remoteOrder receipt -- gas used :',
        contractxOrderReceipt.cumulativeGasUsed.toString()
    );
    const contractxAddSupportedBridgeAssets =
        await contract.addSupportedBridgeAsset(
            '0x0000000000000000000000000000000000000000',
            1000
        ); // 1000 is the bridge asset id for ETH on t0rn
    const contractxAddSupportedBridgeAssetsReceipt =
        await contractxAddSupportedBridgeAssets.wait();
    console.log(
        'Call ContractXOrder.addSupportedBridgeAsset receipt -- gas used :',
        contractxAddSupportedBridgeAssetsReceipt.cumulativeGasUsed.toString()
    );

    // bridge sepl for alice 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
    const contractxBridgeAssets = await contract.remoteBridgeAsset(
        '0x0000000000000000000000000000000000000000',
        '0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d',
        100,
        100,
        { value: ethers.utils.parseUnits('100', 'wei') }
    ); // 1000 is the bridge asset id for ETH on t0rn
    const contractxBridgeAssetsReceipt = await contractxBridgeAssets.wait();
    console.log(
        'Call ContractXOrder.remoteBridgeAsset receipt -- gas used :',
        contractxBridgeAssetsReceipt.cumulativeGasUsed.toString()
    );
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });

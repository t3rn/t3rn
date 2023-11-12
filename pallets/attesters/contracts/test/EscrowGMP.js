// Include the helper libraries
const { ethers } = require('hardhat');
const { expect } = require('chai');

function generateId(addr, nonce) {
    let xtx_id = ethers.utils.keccak256(
        ethers.utils.defaultAbiCoder.encode(
            ['address', 'uint32'],
            [addr, nonce]
        )
    );

    return ethers.utils.keccak256(
        ethers.utils.defaultAbiCoder.encode(
            ['bytes32', 'bytes32'],
            [
                xtx_id,
                '0x0000000000000000000000000000000000000000000000000000000000000000',
            ]
        )
    );
}

function generateRemotePaymentEncoded(
    sender,
    nonce,
    asset,
    rewardAsset,
    amount,
    rewardAmount
) {
    return ethers.utils.defaultAbiCoder.encode(
        ['address', 'uint32', 'address', 'address', 'uint256', 'uint256'],
        [sender, nonce, asset, rewardAsset, amount, rewardAmount]
    );
}

function generateRemotePaymentPayloadEncoded(blockNumber) {
    return ethers.utils.defaultAbiCoder.encode(['uint256'], [blockNumber]);
}

describe('EscrowGMP::storePayloadId', function () {
    let EscrowGMP;
    let contract;
    let owner;
    let addr1;
    let addr2;

    beforeEach(async function () {
        EscrowGMP = await ethers.getContractFactory('EscrowGMP');
        [owner, addr1, addr2, _] = await ethers.getSigners();
        contract = await EscrowGMP.deploy(ethers.constants.AddressZero);
        await contract.deployed();
    });

    it('Should store remote order payload', async function () {
        const encodedPaymentEth = generateRemotePaymentEncoded(
            addr1.address,
            0,
            ethers.constants.AddressZero,
            ethers.constants.AddressZero,
            10,
            20
        );

        const encodedPaymentEthHash = ethers.utils.keccak256(encodedPaymentEth);

        const encodedPayload = generateRemotePaymentPayloadEncoded(1);

        const storeRemoteOrderPayloadCall =
            await contract.storeRemoteOrderPayload(
                encodedPaymentEthHash,
                encodedPayload
            );
        const receipt = await storeRemoteOrderPayloadCall.wait();
        console.log(
            'receipt storeRemoteOrderPayloadCall -- gas used',
            receipt.cumulativeGasUsed.toString()
        );
        // Check the storage entry for the payload
        const payload = await contract.remotePaymentsPayloadHash(
            encodedPaymentEthHash
        );
        expect(payload).to.equal(encodedPayload);
    });

    it('Should not override remote order payload with new payload', async function () {
        const encodedPaymentEth = generateRemotePaymentEncoded(
            addr1.address,
            0,
            ethers.constants.AddressZero,
            ethers.constants.AddressZero,
            10,
            20
        );

        const encodedPaymentEthHash = ethers.utils.keccak256(encodedPaymentEth);

        const encodedPayload = generateRemotePaymentPayloadEncoded(1);

        await contract.storeRemoteOrderPayload(
            encodedPaymentEthHash,
            encodedPayload
        );
        // Check the storage entry for the payload
        const payloadFirst = await contract.remotePaymentsPayloadHash(
            encodedPaymentEthHash
        );
        expect(payloadFirst).to.equal(encodedPayload);
        const encodedPayloadOverrideAttempt =
            generateRemotePaymentPayloadEncoded(2);
        await contract.storeRemoteOrderPayload(
            encodedPaymentEthHash,
            encodedPayloadOverrideAttempt
        );

        // Check the storage entry for the payload
        const payloadFinal = await contract.remotePaymentsPayloadHash(
            encodedPaymentEthHash
        );
        expect(payloadFinal).to.equal(encodedPayload);
    });

    it('commitRemoteBeneficiaryPayload can only be called by attesters', async function () {
        const encodedPaymentEth = generateRemotePaymentEncoded(
            addr1.address,
            0,
            ethers.constants.AddressZero,
            ethers.constants.AddressZero,
            10,
            20
        );

        const encodedPaymentEthHash = ethers.utils.keccak256(encodedPaymentEth);

        const encodedPayload = generateRemotePaymentPayloadEncoded(1);

        await contract.storeRemoteOrderPayload(
            encodedPaymentEthHash,
            encodedPayload
        );
        // Check the storage entry for the payload
        const payloadFirst = await contract.remotePaymentsPayloadHash(
            encodedPaymentEthHash
        );
        expect(payloadFirst).to.equal(encodedPayload);

        // Try calling commitRemoteBeneficiaryPayload as addr2 (not an attester)
        await expect(
            contract
                .connect(addr2)
                .commitRemoteBeneficiaryPayload(
                    encodedPaymentEthHash,
                    addr2.address
                )
        ).to.be.revertedWith('Only Attesters can call this function');

        // Set attesters as addr1
        await contract.assignAttesters(addr1.address);

        // Try calling commitRemoteBeneficiaryPayload as addr1 (an attester)
        await expect(
            contract
                .connect(addr1)
                .commitRemoteBeneficiaryPayload(
                    encodedPaymentEthHash,
                    addr2.address
                )
        ).to.not.be.reverted;
    });

    it('commitRemoteBeneficiaryPayload assigns the executor as beneficiary and user as incapable of claiming it back', async function () {
        const encodedPaymentEth = generateRemotePaymentEncoded(
            addr1.address,
            0,
            ethers.constants.AddressZero,
            ethers.constants.AddressZero,
            10,
            20
        );

        const encodedPaymentEthHash = ethers.utils.keccak256(encodedPaymentEth);

        const encodedPayload = generateRemotePaymentPayloadEncoded(1);

        await contract.storeRemoteOrderPayload(
            encodedPaymentEthHash,
            encodedPayload
        );
        // Check the storage entry for the payload
        const payloadFirst = await contract.remotePaymentsPayloadHash(
            encodedPaymentEthHash
        );
        expect(payloadFirst).to.equal(encodedPayload);

        // Set attesters as addr1
        await contract.assignAttesters(addr1.address);

        // Try calling commitRemoteBeneficiaryPayload as addr1 (an attester)
        await expect(
            contract
                .connect(addr1)
                .commitRemoteBeneficiaryPayload(
                    encodedPaymentEthHash,
                    addr2.address
                )
        ).to.not.be.reverted;
        // Hash the current payload with addr2 as the beneficiary
        // keccak256(abi.encode(currentHash, beneficiary));
        const encodedPaymentEthHashWithBeneficiary = ethers.utils.keccak256(
            encodedPayload + '000000000000000000000000' + addr2.address.slice(2)
        );

        // Check the storage entry for the payload
        const payloadFinal = await contract.remotePaymentsPayloadHash(
            encodedPaymentEthHash
        );
        expect(payloadFinal).to.equal(encodedPaymentEthHashWithBeneficiary);
    });

    it('revertRemoteOrderPayload assigns the empty address as order beneficiary making user capable of claiming rewards back', async function () {
        const encodedPaymentEth = generateRemotePaymentEncoded(
            addr1.address,
            0,
            ethers.constants.AddressZero,
            ethers.constants.AddressZero,
            10,
            20
        );

        const encodedPaymentEthHash = ethers.utils.keccak256(encodedPaymentEth);

        const encodedPayload = generateRemotePaymentPayloadEncoded(1);

        await contract.storeRemoteOrderPayload(
            encodedPaymentEthHash,
            encodedPayload
        );
        // Check the storage entry for the payload
        const payloadFirst = await contract.remotePaymentsPayloadHash(
            encodedPaymentEthHash
        );
        expect(payloadFirst).to.equal(encodedPayload);

        // Set attesters as addr1
        await contract.assignAttesters(addr1.address);

        // Try calling commitRemoteBeneficiaryPayload as addr1 (an attester)
        await expect(
            contract
                .connect(addr1)
                .revertRemoteOrderPayload(encodedPaymentEthHash)
        ).to.not.be.reverted;
        // Hash the current payload with addr2 as the beneficiary
        // keccak256(abi.encode(currentHash, beneficiary));
        const encodedPaymentEthHashWithBeneficiary = ethers.utils.keccak256(
            encodedPayload +
                '0000000000000000000000000000000000000000000000000000000000000000'
        );

        // Check the storage entry for the payload
        const payloadFinal = await contract.remotePaymentsPayloadHash(
            encodedPaymentEthHash
        );
        expect(payloadFinal).to.equal(encodedPaymentEthHashWithBeneficiary);
    });
});

//
// describe("EscrowGMP::Rewards", function() {
//     let contract, USDCContract, owner, addr1, initialAddr1Balance, initialAddr1USDCBalance;
//     before(async function() {
//         await require("hardhat").network.provider.request({
//             method: "hardhat_reset",
//             params: [],
//         });
//         // get signers
//         [owner, addr1] = await ethers.getSigners();
//
//         // deploy contract
//         const EscrowGMP = await ethers.getContractFactory("EscrowGMP");
//         contract = await EscrowGMP.deploy();
//         await contract.deployed();
//
//         // deploy USDC mock contract
//         const ERC20Mock = await ethers.getContractFactory("ERC20Mock");
//         USDCContract = await ERC20Mock.deploy("USD Coin", "USDC");
//         await USDCContract.deployed();
//
//         // mint some USDC for addr1
//         initialAddr1USDCBalance = ethers.utils.parseUnits('1000', 6); // 1000 USDC
//         await USDCContract.mint(addr1.address, initialAddr1USDCBalance);
//
//         // save initial balance of addr1
//         initialAddr1Balance = await ethers.provider.getBalance(addr1.address);
//     });
//
//     // Parameters for the order function
//     const destination = "0x03030303"; // arbitrary destination string
//     const asset = "0x05050505"; // arbitrary asset string
//     const amount = ethers.utils.parseEther('10'); // arbitrary amount of 10 ETH
//     const insurance = ethers.utils.parseEther('2'); // arbitrary insurance of 2 ETH
//     const maxRewardETH = ethers.utils.parseEther('1'); // 1 ETH
//     const maxRewardUSDC = ethers.utils.parseUnits('100', 6); // 100 USDC
//     const rewardAssetETH = ethers.constants.AddressZero; // For ETH
//
//     async function getParams() {
//         const [owner, addr1] = await ethers.getSigners();
//
//         // Parameters for the order function
//         return {
//             destination, // arbitrary destination string
//             asset, // arbitrary asset string
//             targetAccount: addr1.address, // user's address
//             amount, // arbitrary amount of 10 ETH
//             insurance, // arbitrary insurance of 2 ETH
//             maxRewardETH, // 1 ETH
//             maxRewardUSDC, // 100 USDC
//             rewardAssetETH, // For ETH
//             rewardAssetUSDC: USDCContract.address // For USDC
//         };
//     }
//
//     it("Should subtract maxReward correctly in ETH", async function() {
//         // send order with ETH as rewardAsset
//         let params = await getParams();
//         await contract.connect(addr1).order(params.destination, params.asset, params.targetAccount, params.amount, params.rewardAssetETH, params.insurance, params.maxRewardETH,  { value: params.maxRewardETH } );
//
//         // check contract balance
//         const contractBalance = await ethers.provider.getBalance(contract.address);
//         expect(contractBalance).to.equal(maxRewardETH);
//     });
//
//     it("Should subtract maxReward correctly in USDC", async function() {
//         // approve the contract to spend addr1's USDC
//
//         await USDCContract.connect(addr1).approve(contract.address, maxRewardUSDC);
//         // send order with ETH as rewardAsset
//         let params = await getParams();
//         // send order with USDC as rewardAsset
//         await contract.connect(addr1).order(params.destination, params.asset, params.targetAccount, params.amount, params.rewardAssetUSDC, params.insurance, params.maxRewardUSDC);
//
//         // check USDC balance of contract
//         const contractUSDCBalance = await USDCContract.balanceOf(contract.address);
//         expect(contractUSDCBalance).to.equal(maxRewardUSDC);
//     });
//
//     it("Should emit event with correct arguments", async function() {
//         // send order with ETH as rewardAsset
//         let params = await getParams();
//         let sender = await ethers.provider.getSigner(addr1.address)._address;
//         await expect(contract.connect(addr1).order(params.destination, params.asset, params.targetAccount, params.amount, params.rewardAssetETH, params.insurance, params.maxRewardETH, { value: params.maxRewardETH }))
//             .to.emit(contract, 'OrderCreated')
//             // event OrderCreated(bytes32 indexed id, bytes4 indexed destination, bytes4 asset, address targetAccount, uint256 amount, address rewardAsset, uint256 insurance, uint256 maxReward);
//             .withArgs(generateId(sender, 2), params.destination, params.asset, params.targetAccount, params.amount, params.rewardAssetETH, params.insurance, params.maxRewardETH, 2);
//     });
//
//     it("Should set order status to Committed after commit", async function() {
//         // send order with ETH as rewardAsset
//         let params = await getParams();
//         let id = generateId(addr1.address, 1);
//         await contract.connect(addr1).order(params.destination, params.asset, params.targetAccount, params.amount, params.rewardAssetETH, params.insurance, params.maxRewardETH, { value: params.maxRewardETH });
//         await contract.connect(addr1).commit(id);
//
//         let status = await contract.orders(id);
//         expect(status.toString()).to.equal('true,0x03030303,0x05050505,0x70997970C51812dc3A010C7d01b50e0d17dc79C8,0x70997970C51812dc3A010C7d01b50e0d17dc79C8,10000000000000000000,0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512,2000000000000000000,100000000,1'); // 1 corresponds to "Committed"
//     });
//
//     it("Should revert order and refund user in ETH", async function() {
//         let params = await getParams();
//         let id = generateId(addr1.address, 2);
//         await contract.connect(addr1).order(params.destination, params.asset, params.targetAccount, params.amount, params.rewardAssetETH, params.insurance, params.maxRewardETH, { value: params.maxRewardETH });
//         await contract.connect(addr1).revertOrder(id);
//
//         let status = await contract.orders(id);
//         expect(status.toString()).to.equal('true,0x03030303,0x05050505,0x70997970C51812dc3A010C7d01b50e0d17dc79C8,0x70997970C51812dc3A010C7d01b50e0d17dc79C8,10000000000000000000,0x0000000000000000000000000000000000000000,2000000000000000000,1000000000000000000,2'); // 2 corresponds to "reverted
//         let balance = await ethers.provider.getBalance(addr1.address);
//     });
//
//     it("Should revert order and refund user in USDC", async function() {
//         let params = await getParams();
//         await USDCContract.connect(addr1).approve(contract.address, params.maxRewardUSDC);
//         let balancePriorOrder = await USDCContract.balanceOf(addr1.address);
//         let id = generateId(addr1.address, 5);
//         await contract.connect(addr1).order(params.destination, params.asset, params.targetAccount, params.amount, params.rewardAssetUSDC, params.insurance, params.maxRewardUSDC);
//         await expect(contract.connect(addr1).revertOrder(id)).to.emit(contract, 'OrderRefundedInERC20')
//             .withArgs(id, addr1.address, params.maxRewardUSDC);
//
//         let status = await contract.orders(id);
//         expect(status.toString()).to.equal("true,0x03030303,0x05050505,0x70997970C51812dc3A010C7d01b50e0d17dc79C8,0x70997970C51812dc3A010C7d01b50e0d17dc79C8,10000000000000000000,0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512,2000000000000000000,100000000,2"); // 2 corresponds to "Reverted"
//         let balancePost = await USDCContract.balanceOf(addr1.address);
//         expect(balancePriorOrder).to.equal("900000000"); // Ensure the USDC was refunded
//         expect(balancePost).to.equal(balancePriorOrder); // Ensure the USDC was refunded
//     });
//
//     it("Should return false when ID doesn't exist", async function() {
//         let id = ethers.utils.keccak256(ethers.utils.defaultAbiCoder.encode(['address', 'uint32'], [addr1.address, 100])); // Non-existent ID
//         let exists = await contract.isKnownId(id);
//         expect(exists).to.equal(false);
//     });
//
//     it("Should return true when ID exists", async function() {
//         let params = await getParams();
//         let id = generateId(addr1.address, 2);
//         await contract.connect(addr1).order(params.destination, params.asset, params.targetAccount, params.amount, params.rewardAssetETH, params.insurance, params.maxRewardETH, { value: params.maxRewardETH });
//         let exists = await contract.isKnownId(id);
//         expect(exists).to.equal(true);
//     });
// });

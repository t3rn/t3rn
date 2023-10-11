// Include the helper libraries
const { ethers } = require('hardhat');
const { expect } = require('chai');
const {formatBytes32String, solidityPack} = require("ethers/lib/utils");


function generateId(addr, nonce) {
    let xtx_id = ethers.utils.keccak256(ethers.utils.defaultAbiCoder.encode(
      ["address", "uint32"],
      [addr, nonce]
    ));

    return ethers.utils.keccak256(ethers.utils.defaultAbiCoder.encode(
      ["bytes32", "bytes32"],
      [xtx_id, "0x0000000000000000000000000000000000000000000000000000000000000000"]
    ));
}


describe("RemoteOrder::ID", function() {
    let RemoteOrder;
    let contract;
    let owner;
    let addr1;
    let addr2;
    let t3rnVaultContract;
    let escrowGMPContract;

    before(async function () {
        const t3rnVault = await ethers.getContractFactory("t3rnVault");
        t3rnVaultContract = await t3rnVault.deploy();
        const EscrowGMP = await ethers.getContractFactory("EscrowGMP");
        escrowGMPContract = await EscrowGMP.deploy(t3rnVaultContract.address);
        await escrowGMPContract.deployed();
    });

    beforeEach(async function() {
        RemoteOrder = await ethers.getContractFactory("RemoteOrder");
        [owner, addr1, addr2, _] = await ethers.getSigners();
        contract = await RemoteOrder.deploy(escrowGMPContract.address, t3rnVaultContract.address);
        await contract.deployed();
    });

    it("Should generate ID correctly from nonce 0", async function() {
        const id = await contract.generateId(owner.address, 0);
        let expected_id_0 = generateId(owner.address, 0);
        console.log("ethers.utils.defaultAbiCoder.encode([\"address\", \"uint32\"], [owner.address, 0])", expected_id_0)
        expect(id).to.equal(expected_id_0);
    });

    it("Should generate ID correctly from nonce 0, 1 and 2", async function() {
        for (let i = 0; i < 3; i++) {
            const id = await contract.generateId(owner.address, i);
            let expected_id_i = generateId(owner.address, i);
            console.log("ethers.utils.defaultAbiCoder.encode([\"address\", \"uint32\"], [owner.address, 0])", expected_id_i)
            expect(id).to.equal(expected_id_i);
        }
    });


    describe("RemoteOrder::Rewards", function() {
        let contract, USDCContract, owner, addr1, initialAddr1Balance, initialAddr1USDCBalance;
        before(async function() {
            await require("hardhat").network.provider.request({
                method: "hardhat_reset",
                params: [],
            });
            // get signers
            [owner, addr1] = await ethers.getSigners();

            // deploy contract
            const RemoteOrder = await ethers.getContractFactory("RemoteOrder");
            contract = await RemoteOrder.deploy(escrowGMPContract.address, t3rnVaultContract.address);
            await contract.deployed();

            // deploy USDC mock contract
            const ERC20Mock = await ethers.getContractFactory("ERC20Mock");
            USDCContract = await ERC20Mock.deploy("USD Coin", "USDC");
            await USDCContract.deployed();

            // mint some USDC for addr1
            initialAddr1USDCBalance = ethers.utils.parseUnits('1000', 6); // 1000 USDC
            await USDCContract.mint(addr1.address, initialAddr1USDCBalance);

            // save initial balance of addr1
            initialAddr1Balance = await ethers.provider.getBalance(addr1.address);
        });

        // Parameters for the order function
        const destination = "0x03030303"; // arbitrary destination string
        const asset = "0x05050505"; // arbitrary asset string
        const amount = ethers.utils.parseEther('10'); // arbitrary amount of 10 ETH
        const insurance = ethers.utils.parseEther('2'); // arbitrary insurance of 2 ETH
        const maxRewardETH = ethers.utils.parseEther('1'); // 1 ETH
        const maxRewardUSDC = ethers.utils.parseUnits('100', 6); // 100 USDC
        const rewardAssetETH = ethers.constants.AddressZero; // For ETH

        async function getParams() {
            const [owner, addr1] = await ethers.getSigners();

            // Parameters for the order function
            return {
                destination, // arbitrary destination string
                asset, // arbitrary asset string
                targetAccount: formatBytes32String("0x01"), // user's address
                amount, // arbitrary amount of 10 ETH
                insurance, // arbitrary insurance of 2 ETH
                maxRewardETH, // 1 ETH
                maxRewardUSDC, // 100 USDC
                rewardAssetETH, // For ETH
                rewardAssetUSDC: USDCContract.address // For USDC
            };
        }

        it("Should subtract maxReward correctly in ETH", async function() {
            // send order with ETH as rewardAsset
            let params = await getParams();
            await contract.connect(addr1).remoteOrderDecoded(params.destination, params.asset, params.targetAccount, params.amount, params.rewardAssetETH, params.insurance, params.maxRewardETH,  { value: params.maxRewardETH } );

            // check contract balance
            const contractBalance = await ethers.provider.getBalance(t3rnVaultContract.address);
            expect(contractBalance).to.equal(maxRewardETH);
        });

        it.skip("Should subtract maxReward correctly in USDC", async function() {
            // approve the contract to spend addr1's USDC

            await USDCContract.connect(addr1).approve(contract.address, maxRewardUSDC);
            // send order with ETH as rewardAsset
            let params = await getParams();
            // send order with USDC as rewardAsset
            await contract.connect(addr1).remoteOrderDecoded(params.destination, params.asset, params.targetAccount, params.amount, params.rewardAssetUSDC, params.insurance, params.maxRewardUSDC);

            // check USDC balance of contract
            const contractUSDCBalance = await USDCContract.balanceOf(t3rnVaultContract.address);
            expect(contractUSDCBalance).to.equal(maxRewardUSDC);
        });

        it.skip("Should emit event with correct arguments", async function() {
            // send order with ETH as rewardAsset
            let params = await getParams();
            let sender = await ethers.provider.getSigner(addr1.address)._address;

            // Encode without packed
            let encodedParams = ethers.utils.defaultAbiCoder.encode(['bytes4', 'bytes4', 'bytes32', 'uint256', 'address', "uint256", "uint256"], [params.destination, params.asset, params.targetAccount, params.amount, params.rewardAssetETH, params.insurance, params.maxRewardETH])

            await expect(contract.connect(addr1).remoteOrderDecoded(params.destination, params.asset, params.targetAccount, params.amount, params.rewardAssetETH, params.insurance, params.maxRewardETH, { value: params.maxRewardETH }))
                .to.emit(contract, 'RemoteOrderCreated')
                .withArgs(generateId(sender, 10), 10, addr1.address, encodedParams);
        });

        it.skip("Should revert order and refund user in ETH", async function() {
            let params = await getParams();
            let id = generateId(addr1.address, 2);
            await contract.connect(addr1).remoteOrderDecoded(params.destination, params.asset, params.targetAccount, params.amount, params.rewardAssetETH, params.insurance, params.maxRewardETH, { value: params.maxRewardETH });
            await contract.connect(addr1).revertOrder(id);

            let status = await contract.orders(id);
            expect(status.toString()).to.equal('true,0x03030303,0x05050505,0x70997970C51812dc3A010C7d01b50e0d17dc79C8,0x70997970C51812dc3A010C7d01b50e0d17dc79C8,10000000000000000000,0x0000000000000000000000000000000000000000,2000000000000000000,1000000000000000000,2'); // 2 corresponds to "reverted
            let balance = await ethers.provider.getBalance(addr1.address);
        });

        it.skip("Should revert order and refund user in USDC", async function() {
            let params = await getParams();
            await USDCContract.connect(addr1).approve(contract.address, params.maxRewardUSDC);
            let balancePriorOrder = await USDCContract.balanceOf(addr1.address);
            let id = generateId(addr1.address, 5);
            await contract.connect(addr1).remoteOrderDecoded(params.destination, params.asset, params.targetAccount, params.amount, params.rewardAssetUSDC, params.insurance, params.maxRewardUSDC);
            await expect(contract.connect(addr1).revertOrder(id)).to.emit(contract, 'OrderRefundedInERC20')
                .withArgs(id, addr1.address, params.maxRewardUSDC);

            let status = await contract.orders(id);
            expect(status.toString()).to.equal("true,0x03030303,0x05050505,0x70997970C51812dc3A010C7d01b50e0d17dc79C8,0x70997970C51812dc3A010C7d01b50e0d17dc79C8,10000000000000000000,0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512,2000000000000000000,100000000,2"); // 2 corresponds to "Reverted"
            let balancePost = await USDCContract.balanceOf(addr1.address);
            expect(balancePriorOrder).to.equal("900000000"); // Ensure the USDC was refunded
            expect(balancePost).to.equal(balancePriorOrder); // Ensure the USDC was refunded
        });
    });
});



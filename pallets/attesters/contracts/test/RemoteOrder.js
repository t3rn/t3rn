// Include the helper libraries
const { ethers } = require('hardhat');
const { expect } = require('chai');

describe("RemoteOrder::ID", function() {
    let RemoteOrder;
    let contract;
    let owner;
    let addr1;
    let addr2;

    beforeEach(async function() {
        RemoteOrder = await ethers.getContractFactory("RemoteOrder");
        [owner, addr1, addr2, _] = await ethers.getSigners();
        contract = await RemoteOrder.deploy();
        await contract.deployed();
    });

    it("Should generate ID correctly from nonce 0", async function() {
        const id = await contract.generateId(owner.address, 0);
        console.log("ethers.utils.defaultAbiCoder.encode([\"address\", \"uint32\"], [owner.address, 0])", ethers.utils.defaultAbiCoder.encode(["address", "uint32"], [owner.address, 0]))
        expect(id).to.equal(ethers.utils.keccak256(ethers.utils.defaultAbiCoder.encode(["address", "uint32"], [owner.address, 0])));
    });

    it("Should generate ID correctly from nonce 0, 1 and 2", async function() {
        for (let i = 0; i < 3; i++) {
            const id = await contract.generateId(owner.address, i);
            console.log("ethers.utils.defaultAbiCoder.encode([\"address\", \"uint32\"], [owner.address, 0])", ethers.utils.defaultAbiCoder.encode(["address", "uint32"], [owner.address, i]))
            expect(id).to.equal(ethers.utils.keccak256(ethers.utils.defaultAbiCoder.encode(["address", "uint32"], [owner.address, i])));
        }
    });
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
        contract = await RemoteOrder.deploy();
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
            targetAccount: addr1.address, // user's address
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
        await contract.connect(addr1).order(params.destination, params.asset, params.targetAccount, params.amount, params.rewardAssetETH, params.insurance, params.maxRewardETH,  { value: params.maxRewardETH } );

        // check contract balance
        const contractBalance = await ethers.provider.getBalance(contract.address);
        expect(contractBalance).to.equal(maxRewardETH);
    });

    it("Should subtract maxReward correctly in USDC", async function() {
        // approve the contract to spend addr1's USDC

        await USDCContract.connect(addr1).approve(contract.address, maxRewardUSDC);
        // send order with ETH as rewardAsset
        let params = await getParams();
        // send order with USDC as rewardAsset
        await contract.connect(addr1).order(params.destination, params.asset, params.targetAccount, params.amount, params.rewardAssetUSDC, params.insurance, params.maxRewardUSDC);

        // check USDC balance of contract
        const contractUSDCBalance = await USDCContract.balanceOf(contract.address);
        expect(contractUSDCBalance).to.equal(maxRewardUSDC);
    });

    it("Should emit event with correct arguments", async function() {
        // send order with ETH as rewardAsset
        let params = await getParams();
        let sender = await ethers.provider.getSigner(addr1.address)._address;
        await expect(contract.connect(addr1).order(params.destination, params.asset, params.targetAccount, params.amount, params.rewardAssetETH, params.insurance, params.maxRewardETH, { value: params.maxRewardETH }))
            .to.emit(contract, 'OrderCreated')
            // event OrderCreated(bytes32 indexed id, bytes4 indexed destination, bytes4 asset, address targetAccount, uint256 amount, address rewardAsset, uint256 insurance, uint256 maxReward);
            .withArgs(ethers.utils.keccak256(ethers.utils.defaultAbiCoder.encode(["address", "uint32"], [sender, 2])), params.destination, params.asset, params.targetAccount, params.amount, params.rewardAssetETH, params.insurance, params.maxRewardETH, 2);
    });

    it("Should set order status to Committed after commit", async function() {
        // send order with ETH as rewardAsset
        let params = await getParams();
        let id = ethers.utils.keccak256(ethers.utils.defaultAbiCoder.encode(['address', 'uint32'], [addr1.address, 1]));
        await contract.connect(addr1).order(params.destination, params.asset, params.targetAccount, params.amount, params.rewardAssetETH, params.insurance, params.maxRewardETH, { value: params.maxRewardETH });
        await contract.connect(addr1).commit(id);

        let status = await contract.orders(id);
        expect(status.toString()).to.equal('true,0x03030303,0x05050505,0x70997970C51812dc3A010C7d01b50e0d17dc79C8,0x70997970C51812dc3A010C7d01b50e0d17dc79C8,10000000000000000000,0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512,2000000000000000000,100000000,1'); // 1 corresponds to "Committed"
    });

    it("Should revert order and refund user in ETH", async function() {
        let params = await getParams();
        let id = ethers.utils.keccak256(ethers.utils.defaultAbiCoder.encode(['address', 'uint32'], [addr1.address, 2]));
        await contract.connect(addr1).order(params.destination, params.asset, params.targetAccount, params.amount, params.rewardAssetETH, params.insurance, params.maxRewardETH, { value: params.maxRewardETH });
        await contract.connect(addr1).revertOrder(id);

        let status = await contract.orders(id);
        expect(status.toString()).to.equal('true,0x03030303,0x05050505,0x70997970C51812dc3A010C7d01b50e0d17dc79C8,0x70997970C51812dc3A010C7d01b50e0d17dc79C8,10000000000000000000,0x0000000000000000000000000000000000000000,2000000000000000000,1000000000000000000,2'); // 2 corresponds to "reverted
        let balance = await ethers.provider.getBalance(addr1.address);
    });

    it("Should revert order and refund user in USDC", async function() {
        let params = await getParams();
        await USDCContract.connect(addr1).approve(contract.address, params.maxRewardUSDC);
        let balancePriorOrder = await USDCContract.balanceOf(addr1.address);
        let id = ethers.utils.keccak256(ethers.utils.defaultAbiCoder.encode(['address', 'uint32'], [addr1.address, 5]));
        await contract.connect(addr1).order(params.destination, params.asset, params.targetAccount, params.amount, params.rewardAssetUSDC, params.insurance, params.maxRewardUSDC);
        await expect(contract.connect(addr1).revertOrder(id)).to.emit(contract, 'OrderRefundedInERC20')
            .withArgs(ethers.utils.keccak256(ethers.utils.defaultAbiCoder.encode(["address", "uint32"], [addr1.address, 5])), addr1.address, params.maxRewardUSDC);

        let status = await contract.orders(id);
        expect(status.toString()).to.equal("true,0x03030303,0x05050505,0x70997970C51812dc3A010C7d01b50e0d17dc79C8,0x70997970C51812dc3A010C7d01b50e0d17dc79C8,10000000000000000000,0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512,2000000000000000000,100000000,2"); // 2 corresponds to "Reverted"
        let balancePost = await USDCContract.balanceOf(addr1.address);
        expect(balancePriorOrder).to.equal("900000000"); // Ensure the USDC was refunded
        expect(balancePost).to.equal(balancePriorOrder); // Ensure the USDC was refunded
    });

    it("Should return false when ID doesn't exist", async function() {
        let id = ethers.utils.keccak256(ethers.utils.defaultAbiCoder.encode(['address', 'uint32'], [addr1.address, 100])); // Non-existent ID
        let exists = await contract.isKnownId(id);
        expect(exists).to.equal(false);
    });

    it("Should return true when ID exists", async function() {
        let params = await getParams();
        let id = ethers.utils.keccak256(ethers.utils.defaultAbiCoder.encode(['address', 'uint32'], [addr1.address, 2]));
        await contract.connect(addr1).order(params.destination, params.asset, params.targetAccount, params.amount, params.rewardAssetETH, params.insurance, params.maxRewardETH, { value: params.maxRewardETH });
        let exists = await contract.isKnownId(id);
        expect(exists).to.equal(true);
    });
});

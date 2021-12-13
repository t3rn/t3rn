const { expect, assert } = require("chai");
const { ethers } = require("hardhat");
const crypto = require('crypto');

// const 
const BN = require('bn.js');
let escrow;
let token
let executor;
let from
let to;

describe("Escrow", function () {

	before(async () => {
			const [exec, sender, receiver] = await ethers.getSigners();
			executor = exec
			from = sender
			to = receiver
			const Token = await hre.ethers.getContractFactory("_ERC20");

			// deploy tokens for testing
			token = await Token.connect(executor).deploy("token", "tok");
			await token.deployed()

			// approve router for liquidity provisioning
			// await token1.connect(lp).approve("0x7a250d5630b4cf539739df2c5dacb4c659f2488d", ethers.utils.parseEther("2000"))
			// await token2.connect(lp).approve("0x7a250d5630b4cf539739df2c5dacb4c659f2488d", ethers.utils.parseEther("1000"))

			

		})

	it("should depoy contract", async () => {
		const Escrow = await hre.ethers.getContractFactory("Escrow");
		escrow = await Escrow.deploy();

		console.log("Deployed Contract:", escrow.address);

	})

	it("Should initialze eth transfer", async () => {
		const id = "0x" + crypto.createHash('sha256').update("ethTransfer").digest('hex');
		const amount = ethers.utils.parseEther("1");
		await escrow.connect(executor).ethTransfer(to.address, id, {value: amount})
		const contractBalance = await escrow.provider.getBalance(escrow.address)
		assert.ok(contractBalance.toString() === "1000000000000000000")
	});

	it("Should commit transfer", async () => {
		const userBalancePre = await escrow.provider.getBalance(to.address);
		const id = "0x" + crypto.createHash('sha256').update("ethTransfer").digest('hex');
		const amount = ethers.utils.parseEther("1");
		await escrow.connect(executor).releaseEthTransfer({xtxId: id, shouldCommit: true}, to.address, amount);
		const userBalancePost = await escrow.provider.getBalance(to.address);
		console.log(userBalancePost)
		assert.ok(userBalancePre.add(amount).toJSON().hex === userBalancePost.toJSON().hex)
	})

	it("Should revert transfer", async () => {
		// doesnt work yet, for some reason refund is not showing, but also not in contract.
		const receiverBalancePre = await escrow.provider.getBalance(to.address);
		const executorBalancePre = await escrow.provider.getBalance(executor.address);
		const id = "0x" + crypto.createHash('sha256').update("ethTransfer1").digest('hex');
		const amount = ethers.utils.parseEther("1");
		await escrow.connect(executor).ethTransfer(to.address, id, {value: amount})
		await escrow.connect(executor).releaseEthTransfer({xtxId: id, shouldCommit: false}, to.address, amount);
		const receiverBalancePost = await escrow.provider.getBalance(to.address);
		const executorBalancePost = await escrow.provider.getBalance(executor.address);
		assert.ok(receiverBalancePre.toJSON().hex === receiverBalancePost.toJSON().hex)
		assert.ok(executorBalancePre.add(amount).toJSON().hex === executorBalancePost.toJSON().hex)
	})
});
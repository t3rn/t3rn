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
		await escrow.connect(executor).applyEthTransfer({xtxId: id, shouldCommit: true}, to.address, amount);
		const userBalancePost = await escrow.provider.getBalance(to.address);
		assert.ok(userBalancePre.add(amount).toJSON().hex === userBalancePost.toJSON().hex)
	})

	it("Should revert transfer", async () => {
		const receiverBalancePre = await escrow.provider.getBalance(to.address);
		const id = "0x" + crypto.createHash('sha256').update("ethTransfer1").digest('hex');
		const amount = ethers.utils.parseEther("1");

		await escrow.connect(executor).ethTransfer(to.address, id, {value: amount})
		const executorBalancePre = await escrow.provider.getBalance(executor.address);

		await escrow.connect(executor).applyEthTransfer({xtxId: id, shouldCommit: false}, to.address, amount);

		const receiverBalancePost = await escrow.provider.getBalance(to.address);
		const executorBalancePost = await escrow.provider.getBalance(executor.address);
		assert.ok(receiverBalancePre.toJSON().hex === receiverBalancePost.toJSON().hex)
		// we're dividing here to round away the gas cost of executor
		assert.ok(executorBalancePre.add(amount).div(ethers.BigNumber.from(1000000000000000)).toJSON().hex === executorBalancePost.div(ethers.BigNumber.from(1000000000000000)).toJSON().hex)
	})

	it("Should initialze token transfer", async () => {
		// executor must approve contract
		await token.connect(executor).approve(escrow.address, ethers.utils.parseEther("100"))
		const id = "0x" + crypto.createHash('sha256').update("tokenTransfer").digest('hex');
		const amount = ethers.utils.parseEther("100");
		await escrow.connect(executor).tokenTransfer(to.address, token.address, amount, id)
		const contractBalance = await token.balanceOf(escrow.address)
		assert.ok(contractBalance.toString() === "100000000000000000000")
	});

	it("Should commit token transfer", async () => {
		const userBalancePre = await token.balanceOf(to.address)
		const id = "0x" + crypto.createHash('sha256').update("tokenTransfer").digest('hex');
		const amount = ethers.utils.parseEther("100");
		await escrow.connect(executor).applyTokenTransfer({xtxId: id, shouldCommit: true}, to.address, token.address, amount);
		const userBalancePost = await token.balanceOf(to.address)
		assert.ok(userBalancePre.add(amount).toJSON().hex === userBalancePost.toJSON().hex)
	})

	it("Should revert token transfer", async () => {
		const receiverBalancePre = await token.balanceOf(to.address)
		await token.connect(executor).approve(escrow.address, ethers.utils.parseEther("100"))
		const id = "0x" + crypto.createHash('sha256').update("tokenTransfer1").digest('hex');
		const amount = ethers.utils.parseEther("1");

		await escrow.connect(executor).tokenTransfer(to.address, token.address, amount, id)
		const executorBalancePre = await token.balanceOf(executor.address)

		await escrow.connect(executor).applyTokenTransfer({xtxId: id, shouldCommit: false}, to.address, token.address, amount);

		const receiverBalancePost = await token.balanceOf(to.address)
		const executorBalancePost = await token.balanceOf(executor.address)
		assert.ok(receiverBalancePre.toJSON().hex === receiverBalancePost.toJSON().hex)
		assert.ok(executorBalancePre.add(amount).toJSON().hex === executorBalancePost.toJSON().hex)
	})

});
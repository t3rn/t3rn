const { expect, assert } = require("chai");
const { ethers } = require("hardhat");
const crypto = require('crypto');

// const 
const BN = require('bn.js');
let escrow;
let token
let token1
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
		const Token1 = await hre.ethers.getContractFactory("_ERC20");

		// deploy tokens for testing
		token = await Token.connect(executor).deploy("token", "tok");
		await token.deployed()
		token1 = await Token1.connect(executor).deploy("token1", "tok1");
		await token1.deployed()

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
		await escrow.connect(executor).settleEthTransfer({xtxId: id, shouldCommit: true}, to.address, amount);
		const userBalancePost = await escrow.provider.getBalance(to.address);
		assert.ok(userBalancePre.add(amount).toJSON().hex === userBalancePost.toJSON().hex)
	})

	it("Should revert transfer", async () => {
		const receiverBalancePre = await escrow.provider.getBalance(to.address);
		const id = "0x" + crypto.createHash('sha256').update("ethTransfer1").digest('hex');
		const amount = ethers.utils.parseEther("1");

		await escrow.connect(executor).ethTransfer(to.address, id, {value: amount})
		const executorBalancePre = await escrow.provider.getBalance(executor.address);

		await escrow.connect(executor).settleEthTransfer({xtxId: id, shouldCommit: false}, to.address, amount);

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
		await escrow.connect(executor).settleTokenTransfer({xtxId: id, shouldCommit: true}, to.address, token.address, amount);
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

		await escrow.connect(executor).settleTokenTransfer({xtxId: id, shouldCommit: false}, to.address, token.address, amount);

		const receiverBalancePost = await token.balanceOf(to.address)
		const executorBalancePost = await token.balanceOf(executor.address)
		assert.ok(receiverBalancePre.toJSON().hex === receiverBalancePost.toJSON().hex)
		assert.ok(executorBalancePre.add(amount).toJSON().hex === executorBalancePost.toJSON().hex)
	})

	it("Should initialze ethSwap", async () => {
		const id = "0x" + crypto.createHash('sha256').update("ethSwap").digest('hex');
		const amount = ethers.utils.parseEther("1");
		await escrow.connect(executor).ethSwap(to.address, id, {value: amount})
		const contractBalance = await escrow.provider.getBalance(escrow.address)
		assert.ok(contractBalance.toString() === "1000000000000000000")
	});

	it("Should commit ethSwap", async () => {
		const userBalancePre = await escrow.provider.getBalance(to.address);
		const id = "0x" + crypto.createHash('sha256').update("ethSwap").digest('hex');
		const amount = ethers.utils.parseEther("1");
		await escrow.connect(executor).settleEthSwap({xtxId: id, shouldCommit: true}, to.address, amount);
		const userBalancePost = await escrow.provider.getBalance(to.address);
		assert.ok(userBalancePre.add(amount).toJSON().hex === userBalancePost.toJSON().hex)
	})

	it("Should revert ethSwap", async () => {
		const receiverBalancePre = await escrow.provider.getBalance(to.address);
		const id = "0x" + crypto.createHash('sha256').update("ethSwap1").digest('hex');
		const amount = ethers.utils.parseEther("0");

		await escrow.connect(executor).ethSwap(to.address, id, {value: amount})
		const executorBalancePre = await escrow.provider.getBalance(executor.address);

		const tx = await escrow.connect(executor).settleEthSwap({xtxId: id, shouldCommit: false}, to.address, amount);
		const receipt = await tx.wait()
		const gasUsage = receipt.cumulativeGasUsed.mul(receipt.effectiveGasPrice)
		const receiverBalancePost = await escrow.provider.getBalance(to.address);
		const executorBalancePost = await escrow.provider.getBalance(executor.address);
		// console.log("Pre:", executorBalancePre)
		// console.log("post", executorBalancePost)
		// console.log("adding...")
		// console.log("amount:", amount)
		// console.log("Gas:", gasUsage)
		// console.log("pre: ", executorBalancePre.add(amount).add(gasUsage))
		// console.log("post:", executorBalancePost)

		// console.log("difference:", executorBalancePre.add(amount).add(gasUsage).sub(executorBalancePost))

		// console.log(amount)
		console.log("TODO: Fix refund test")
		assert.ok(receiverBalancePre.toJSON().hex === receiverBalancePost.toJSON().hex)
		// assert.ok(executorBalancePre.add(amount).add(gasUsage).toJSON().hex === executorBalancePost.toJSON().hex)
	})


	it("Should initialze tokenSwap", async () => {
		// executor must approve contract
		await token.connect(executor).approve(escrow.address, ethers.utils.parseEther("100"))
		const id = "0x" + crypto.createHash('sha256').update("tokenSwap").digest('hex');
		const amount = ethers.utils.parseEther("100");
		await escrow.connect(executor).tokenSwap(to.address, token.address, amount, id)
		const contractBalance = await token.balanceOf(escrow.address)
		assert.ok(contractBalance.toString() === "100000000000000000000")
	});

	it("Should commit token swap", async () => {
		const userBalancePre = await token.balanceOf(to.address)
		const id = "0x" + crypto.createHash('sha256').update("tokenSwap").digest('hex');
		const amount = ethers.utils.parseEther("100");
		await escrow.connect(executor).settleTokenSwap({xtxId: id, shouldCommit: true}, to.address, token.address, amount);
		const userBalancePost = await token.balanceOf(to.address)
		assert.ok(userBalancePre.add(amount).toJSON().hex === userBalancePost.toJSON().hex)
	})

	it("Should revert token transfer", async () => {
		const receiverBalancePre = await token.balanceOf(to.address)
		await token.connect(executor).approve(escrow.address, ethers.utils.parseEther("100"))
		const id = "0x" + crypto.createHash('sha256').update("tokenSwap1").digest('hex');
		const amount = ethers.utils.parseEther("1");

		await escrow.connect(executor).tokenSwap(to.address, token.address, amount, id)
		const executorBalancePre = await token.balanceOf(executor.address)

		await escrow.connect(executor).settleTokenSwap({xtxId: id, shouldCommit: false}, to.address, token.address, amount);

		const receiverBalancePost = await token.balanceOf(to.address)
		const executorBalancePost = await token.balanceOf(executor.address)
		assert.ok(receiverBalancePre.toJSON().hex === receiverBalancePost.toJSON().hex)
		assert.ok(executorBalancePre.add(amount).toJSON().hex === executorBalancePost.toJSON().hex)
	})

	it("Should initialze addLiquidity", async () => {
		// executor must approve contract
		await token.connect(executor).approve(escrow.address, ethers.utils.parseEther("100")) // we're using the token here we already have. In prod this would be lp token
		const id = "0x" + crypto.createHash('sha256').update("addLiquidity").digest('hex');
		const amount = ethers.utils.parseEther("100");
		await escrow.connect(executor).addLiquidity(to.address, token.address, amount, id)
		const contractBalance = await token.balanceOf(escrow.address)
		assert.ok(contractBalance.toString() === "100000000000000000000")
	});

	it("Should commit addLiquidity", async () => {
		const userBalancePre = await token.balanceOf(to.address)
		const id = "0x" + crypto.createHash('sha256').update("addLiquidity").digest('hex');
		const amount = ethers.utils.parseEther("100");
		await escrow.connect(executor).settleAddLiquidity({xtxId: id, shouldCommit: true}, to.address, token.address, amount);
		const userBalancePost = await token.balanceOf(to.address)
		assert.ok(userBalancePre.add(amount).toJSON().hex === userBalancePost.toJSON().hex)
	})

	it("Should revert addLiquidity", async () => {
		const receiverBalancePre = await token.balanceOf(to.address)
		await token.connect(executor).approve(escrow.address, ethers.utils.parseEther("100"))
		const id = "0x" + crypto.createHash('sha256').update("addLiquidity1").digest('hex');
		const amount = ethers.utils.parseEther("1");

		await escrow.connect(executor).addLiquidity(to.address, token.address, amount, id)
		const executorBalancePre = await token.balanceOf(executor.address)

		await escrow.connect(executor).settleAddLiquidity({xtxId: id, shouldCommit: false}, to.address, token.address, amount);

		const receiverBalancePost = await token.balanceOf(to.address)
		const executorBalancePost = await token.balanceOf(executor.address)
		assert.ok(receiverBalancePre.toJSON().hex === receiverBalancePost.toJSON().hex)
		assert.ok(executorBalancePre.add(amount).toJSON().hex === executorBalancePost.toJSON().hex)
	})

	it("Should initialze removeLiquidity", async () => {
		// executor must approve contract
		await token.connect(executor).approve(escrow.address, ethers.utils.parseEther("100")) // we're using the token here we already have. In prod this would be lp token
		await token1.connect(executor).approve(escrow.address, ethers.utils.parseEther("100"))
		const id = "0x" + crypto.createHash('sha256').update("removeLiquidity").digest('hex');
		const amountA = ethers.utils.parseEther("100");
		const amountB = ethers.utils.parseEther("100");
		await escrow.connect(executor).removeLiquidity(to.address, token.address, token1.address, amountA, amountB, id)
		const contractBalanceA = await token.balanceOf(escrow.address)
		const contractBalanceB = await token.balanceOf(escrow.address)
		assert.ok(contractBalanceA.toString() === "100000000000000000000")
		assert.ok(contractBalanceB.toString() === "100000000000000000000")
	});
	
	it("Should commit removeLiquidity", async () => {
		const userBalanceAPre = await token.balanceOf(to.address)
		const userBalanceBPre = await token1.balanceOf(to.address)
		const id = "0x" + crypto.createHash('sha256').update("removeLiquidity").digest('hex');
		const amountA = ethers.utils.parseEther("100");
		const amountB = ethers.utils.parseEther("100");
		await escrow.connect(executor).settleRemoveLiquidity({xtxId: id, shouldCommit: true}, to.address, token.address, token1.address, amountA, amountB);
		const userBalanceAPost = await token.balanceOf(to.address)
		const userBalanceBPost = await token1.balanceOf(to.address)
		assert.ok(userBalanceAPre.add(amountA).toJSON().hex === userBalanceAPost.toJSON().hex)
		assert.ok(userBalanceBPre.add(amountB).toJSON().hex === userBalanceBPost.toJSON().hex)
	})

	it("Should revert removeLiquidity", async () => {
		await token.connect(executor).approve(escrow.address, ethers.utils.parseEther("100")) // we're using the token here we already have. In prod this would be lp token
		await token1.connect(executor).approve(escrow.address, ethers.utils.parseEther("100"))
		const id = "0x" + crypto.createHash('sha256').update("removeLiquidity1").digest('hex');
		const amountA = ethers.utils.parseEther("100");
		const amountB = ethers.utils.parseEther("100");
		await escrow.connect(executor).removeLiquidity(to.address, token.address, token1.address, amountA, amountB, id)
		const executorBalanceAPre = await token.balanceOf(executor.address)
		const executorBalanceBPre = await token1.balanceOf(executor.address)
		await escrow.connect(executor).settleRemoveLiquidity({xtxId: id, shouldCommit: false}, to.address, token.address, token1.address, amountA, amountB);
		const executorBalanceAPost = await token.balanceOf(executor.address)
		const executorBalanceBPost = await token1.balanceOf(executor.address)
		assert.ok(executorBalanceAPre.add(amountA).toJSON().hex === executorBalanceAPost.toJSON().hex)
		assert.ok(executorBalanceBPre.add(amountB).toJSON().hex === executorBalanceBPost.toJSON().hex)
	})

});
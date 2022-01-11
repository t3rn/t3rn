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

	it("Should initialze executeEth", async () => {
		const id = "0x" + crypto.createHash('sha256').update("executeEth").digest('hex');
		const amount = ethers.utils.parseEther("1");
		const tx = await escrow.connect(executor).executeEth(id, to.address, {value: amount})
		const event = (await tx.wait()).events.find(event => event.event === 'ExecuteEth');
		const contractBalance = await escrow.provider.getBalance(escrow.address)

		assert.ok(contractBalance.toString() === "1000000000000000000")
		assert.ok(event.args.xtxId === id)
		assert.ok(event.args.executor === executor.address)
		assert.ok(event.args.to === to.address)
		assert.ok(event.args.amount.toJSON().hex === amount.toJSON().hex)
	});

	it("Should commit executeEth", async () => {
		const userBalancePre = await escrow.provider.getBalance(to.address);
		const id = "0x" + crypto.createHash('sha256').update("executeEth").digest('hex');
		const amount = ethers.utils.parseEther("1");
		const tx = await escrow.connect(executor).settleEth({xtxId: id, shouldCommit: true}, to.address, amount);
		const event = (await tx.wait()).events.find(event => event.event === 'Commit');
		const userBalancePost = await escrow.provider.getBalance(to.address);
		
		assert.ok(event.args.xtxId === id)
		assert.ok(userBalancePre.add(amount).toJSON().hex === userBalancePost.toJSON().hex)
	})

	it("Should revert executeEth", async () => {
		const id = "0x" + crypto.createHash('sha256').update("executeEth1").digest('hex');
		const amount = ethers.utils.parseEther("1");
		const tx = await escrow.connect(executor).executeEth(id, to.address, {value: amount})
		const contractBalancePre = await escrow.provider.getBalance(escrow.address)
		const executorBalancePre = await escrow.provider.getBalance(executor.address)
		const receiverBalancePre = await escrow.provider.getBalance(to.address);

		await escrow.connect(executor).settleEth({xtxId: id, shouldCommit: false}, to.address, amount);

		const contractBalancePost = await escrow.provider.getBalance(escrow.address)
		const executorBalancePost = await escrow.provider.getBalance(executor.address)
		const receiverBalancePost = await escrow.provider.getBalance(to.address);

		assert.ok(receiverBalancePre.toJSON().hex === receiverBalancePost.toJSON().hex)
		assert.ok((contractBalancePre.sub(amount)).eq(contractBalancePost))
		// we should check the exeact amount here. A bit diffecult because of gas refund though
		assert.ok(executorBalancePre.lt(executorBalancePost))
	})

	it("Should initialze executeToken", async () => {
		const id = "0x" + crypto.createHash('sha256').update("executeToken").digest('hex');
		const amount = ethers.utils.parseEther("100");
		await token.connect(executor).approve(escrow.address, amount)
		const tx = await escrow.connect(executor).executeToken(id, to.address, token.address, amount)
		const event = (await tx.wait()).events.find(event => event.event === 'ExecuteToken');
		const contractBalance = await token.balanceOf(escrow.address)

		assert.ok(contractBalance.toString() === "100000000000000000000")
		assert.ok(event.args.xtxId === id)
		assert.ok(event.args.executor === executor.address)
		assert.ok(event.args.to === to.address)
		assert.ok(event.args.token === token.address)
		assert.ok(event.args.amount.toJSON().hex === amount.toJSON().hex)
	});
	
	it("Should commit executeToken", async () => {
		const userBalancePre = await token.balanceOf(to.address);
		const id = "0x" + crypto.createHash('sha256').update("executeToken").digest('hex');
		const amount = ethers.utils.parseEther("100");
		const tx = await escrow.connect(executor).settleToken({xtxId: id, shouldCommit: true}, to.address, token.address, amount);
		const event = (await tx.wait()).events.find(event => event.event === 'Commit');
		const userBalancePost = await token.balanceOf(to.address);
		
		assert.ok(event.args.xtxId === id)
		assert.ok(userBalancePre.add(amount).toJSON().hex === userBalancePost.toJSON().hex)
	})

	it("Should revert executeToken", async () => {
		const id = "0x" + crypto.createHash('sha256').update("executeToken1").digest('hex');
		const amount = ethers.utils.parseEther("100");
		await token.connect(executor).approve(escrow.address, amount)
		const tx = await escrow.connect(executor).executeToken(id, to.address, token.address, amount)
		const contractBalancePre = await token.balanceOf(escrow.address)
		const executorBalancePre = await token.balanceOf(executor.address)
		const receiverBalancePre = await token.balanceOf(to.address);

		await escrow.connect(executor).settleToken({xtxId: id, shouldCommit: false}, to.address, token.address, amount);

		const contractBalancePost = await token.balanceOf(escrow.address)
		const executorBalancePost = await token.balanceOf(executor.address)
		const receiverBalancePost = await token.balanceOf(to.address);

		assert.ok(receiverBalancePre.eq(receiverBalancePost))
		assert.ok((contractBalancePre.sub(amount)).eq(contractBalancePost))
		assert.ok(executorBalancePre.add(amount).eq(executorBalancePost))
	})

	it("Should initialize removeLiquidity", async () => {
		const amountA = ethers.utils.parseEther("100");
		const amountB = ethers.utils.parseEther("100");
		// executor must approve contract
		await token.connect(executor).approve(escrow.address, amountA) // we're using the token here we already have. In prod this would be lp token
		await token1.connect(executor).approve(escrow.address, amountB)
		const id = "0x" + crypto.createHash('sha256').update("removeLiquidity").digest('hex');
		const tx = await escrow.connect(executor).removeLiquidity(id, to.address, token.address, token1.address, amountA, amountB)
		const contractBalanceA = await token.balanceOf(escrow.address)
		const contractBalanceB = await token.balanceOf(escrow.address)
		const event = (await tx.wait()).events.find(event => event.event === 'ExecuteRemoveLiquidity');

		assert.ok(event.args.xtxId === id)
		assert.ok(event.args.executor === executor.address)
		assert.ok(event.args.to === to.address)
		assert.ok(event.args.tokenA === token.address)
		assert.ok(event.args.tokenB === token1.address)
		assert.ok(event.args.amountA.eq(amountA))
		assert.ok(event.args.amountA.eq(amountB))
		assert.ok(contractBalanceA.toString() === "100000000000000000000")
		assert.ok(contractBalanceB.toString() === "100000000000000000000")
	});
	
	it("Should commit removeLiquidity", async () => {
		const userBalanceAPre = await token.balanceOf(to.address)
		const userBalanceBPre = await token1.balanceOf(to.address)
		const id = "0x" + crypto.createHash('sha256').update("removeLiquidity").digest('hex');
		const amountA = ethers.utils.parseEther("100");
		const amountB = ethers.utils.parseEther("100");
		const tx = await escrow.connect(executor).settleRemoveLiquidity({xtxId: id, shouldCommit: true}, to.address, token.address, token1.address, amountA, amountB);
		const event = (await tx.wait()).events.find(event => event.event === 'Commit');
		const userBalanceAPost = await token.balanceOf(to.address)
		const userBalanceBPost = await token1.balanceOf(to.address)
		assert.ok(userBalanceAPre.add(amountA).toJSON().hex === userBalanceAPost.toJSON().hex)
		assert.ok(userBalanceBPre.add(amountB).toJSON().hex === userBalanceBPost.toJSON().hex)
		assert.ok(event.args.xtxId === id)
	})

	it("Should revert removeLiquidity", async () => {
		const amountA = ethers.utils.parseEther("100");
		const amountB = ethers.utils.parseEther("100");
		await token.connect(executor).approve(escrow.address, amountA) // we're using the token here we already have. In prod this would be lp token
		await token1.connect(executor).approve(escrow.address, amountB)
		const id = "0x" + crypto.createHash('sha256').update("removeLiquidity1").digest('hex');
		await escrow.connect(executor).removeLiquidity(id, to.address, token.address, token1.address, amountA, amountB)

		const contractBalanceAPre = await token.balanceOf(escrow.address)
		const executorBalanceAPre = await token.balanceOf(executor.address)
		const receiverBalanceAPre = await token.balanceOf(to.address);
		const contractBalanceBPre = await token1.balanceOf(escrow.address)
		const executorBalanceBPre = await token1.balanceOf(executor.address)
		const receiverBalanceBPre = await token1.balanceOf(to.address);

		const tx = await escrow.connect(executor).settleRemoveLiquidity({xtxId: id, shouldCommit: false}, to.address, token.address, token1.address, amountA, amountB);
		const event = (await tx.wait()).events.find(event => event.event === 'Revert');
		const contractBalanceAPost = await token.balanceOf(escrow.address)	
		const executorBalanceAPost = await token.balanceOf(executor.address)
		const receiverBalanceAPost = await token.balanceOf(to.address);
		const contractBalanceBPost = await token1.balanceOf(escrow.address)
		const executorBalanceBPost = await token1.balanceOf(executor.address)
		const receiverBalanceBPost = await token1.balanceOf(to.address);

		assert.ok(contractBalanceAPost.add(amountA).eq(contractBalanceAPre))
		assert.ok(contractBalanceBPost.add(amountB).eq(contractBalanceBPre))
		assert.ok(executorBalanceAPre.add(amountA).eq(executorBalanceAPost))
		assert.ok(executorBalanceBPre.add(amountB).eq(executorBalanceBPost))
		assert.ok(receiverBalanceAPre.eq(receiverBalanceAPost))
		assert.ok(receiverBalanceBPre.eq(receiverBalanceBPost))
		assert.ok(event.args.xtxId === id)

	})

});
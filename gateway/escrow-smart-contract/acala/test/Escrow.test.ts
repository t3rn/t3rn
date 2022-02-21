import { expect, use, assert } from 'chai';
import { deployContract, solidity } from 'ethereum-waffle';
import { ethers, Contract, BigNumber } from 'ethers';
import { evmChai, Provider, Signer } from '@acala-network/bodhi';
import { getTestProvider, getTestSigners } from '../utils/provider';
import ADDRESS from '@acala-network/contracts/utils/Address';
import Escrow from '../build/Escrow.json';

use(solidity);
use(evmChai);
const crypto = require('crypto');
const ERC20_ABI = require('@acala-network/contracts/build/contracts/Token.json').abi;


describe('Escrow', () => {
  const amountAUSD = BigNumber.from('100000000000000') // 100 without decimals
  const amountDOT = BigNumber.from('1000000000000') // 100 without decimals
  let executor: Signer;
  let receiver: Signer;
  let escrow: Contract;
  let provider: Provider;
  let ausd: Contract;
  let dot: Contract;
  
  before(async () => {
    provider = await getTestProvider();
    let [exec, receiv] = getTestSigners(provider);
    executor = exec;
    receiver = receiv;
    escrow = await deployContract(executor as any, Escrow)
    console.log('Escrow deployed at:', escrow.address);
    ausd = new ethers.Contract(ADDRESS.AUSD, ERC20_ABI, executor as any)
    dot = new ethers.Contract(ADDRESS.DOT, ERC20_ABI, executor as any)
    console.log('Decimals DOT:', await dot.decimals())
    console.log('Decimals AUSD:', await ausd.decimals())
    console.log()
    console.log('Executor:')
    console.log('DOT Balance:', (await dot.balanceOf(executor.queryEvmAddress())).toString())
    console.log('AUSD Balance:', (await ausd.balanceOf(executor.queryEvmAddress())).toString())
    console.log()
    console.log('Receiver:')
    console.log('DOT Balance:', (await dot.balanceOf(receiver.queryEvmAddress())).toString())
    console.log('AUSD Balance:', (await ausd.balanceOf(receiver.queryEvmAddress())).toString())
    console.log()
  });

  after(async () => {
    provider.api.disconnect();
  });

  it("Should initialze execute", async () => {
		const id = "0x" + crypto.createHash('sha256').update("execute").digest('hex');
		await ausd.connect(executor as any).approve(escrow.address, amountAUSD)
		await expect(escrow.execute(id, receiver.queryEvmAddress(), ausd.address, amountAUSD)).to.emit(escrow, "Execute")
		const contractBalance = await ausd.balanceOf(escrow.address)
		expect(contractBalance).to.equal(amountAUSD);
	});

  it("should commit", async () => {
    const userBalancePre = await ausd.balanceOf(receiver.queryEvmAddress());
    const id = "0x" + crypto.createHash('sha256').update("execute").digest('hex');
    await expect(escrow.settle({xtxId: id, shouldCommit: true}, receiver.queryEvmAddress(), ausd.address, amountAUSD)).to.emit(escrow, "Commit")
		const userBalancePost = await ausd.balanceOf(receiver.queryEvmAddress());
		assert.ok(userBalancePre.add(amountAUSD).eq(userBalancePost))
  })

  it('should revert', async () => {
    const userBalancePre = await ausd.balanceOf(receiver.queryEvmAddress());
    const executorBalancePre = await ausd.balanceOf(executor.queryEvmAddress());
    const id = "0x" + crypto.createHash('sha256').update("execute1").digest('hex');
		await ausd.connect(executor as any).approve(escrow.address, amountAUSD)
		await expect(escrow.execute(id, receiver.queryEvmAddress(), ausd.address, amountAUSD)).to.emit(escrow, "Execute")
		const contractBalancePre = await ausd.balanceOf(escrow.address)
		expect(contractBalancePre).to.equal(amountAUSD);
    await expect(escrow.settle({xtxId: id, shouldCommit: false}, receiver.queryEvmAddress(), ausd.address, amountAUSD)).to.emit(escrow, "Revert")
    const contractBalancePost = await ausd.balanceOf(escrow.address)
		const userBalancePost = await ausd.balanceOf(receiver.queryEvmAddress());
    const executorBalancePost = await ausd.balanceOf(executor.queryEvmAddress());
		assert.ok(userBalancePre.eq(userBalancePost))
    expect(contractBalancePost).to.equal(contractBalancePre.sub(amountAUSD))
    expect(executorBalancePost).to.eq(executorBalancePre)
  })

  it("should initialze removeLiquidity", async () => {
    const id = "0x" + crypto.createHash('sha256').update("executeRemoveLiquidity").digest('hex');
		await ausd.connect(executor as any).approve(escrow.address, amountAUSD)
    await dot.connect(executor as any).approve(escrow.address, amountDOT)
		await expect(escrow.executeRemoveLiquidity(id, receiver.queryEvmAddress(), ausd.address, dot.address, amountAUSD, amountDOT)).to.emit(escrow, "ExecuteRemoveLiquidity")
		const contractBalanceAUSD = await ausd.balanceOf(escrow.address)
		expect(contractBalanceAUSD).to.equal(amountAUSD);
    const contractBalanceDOT = await dot.balanceOf(escrow.address)
		expect(contractBalanceDOT).to.equal(amountDOT);
  })

  it("should commit removeLiquidity", async () => {
    const userBalancePreAUSD = await ausd.balanceOf(receiver.queryEvmAddress());
    const userBalancePreDOT = await dot.balanceOf(receiver.queryEvmAddress());
    const id = "0x" + crypto.createHash('sha256').update("executeRemoveLiquidity").digest('hex');
    await expect(escrow.settleRemoveLiquidity({xtxId: id, shouldCommit: true}, receiver.queryEvmAddress(), ausd.address, dot.address, amountAUSD, amountDOT)).to.emit(escrow, "Commit")
		const userBalancePostAUSD = await ausd.balanceOf(receiver.queryEvmAddress());
    const userBalancePostDOT = await dot.balanceOf(receiver.queryEvmAddress());
		assert.ok(userBalancePreAUSD.add(amountAUSD).eq(userBalancePostAUSD))
		assert.ok(userBalancePreDOT.add(amountDOT).eq(userBalancePostDOT))

  })

  it("should revert removeLiquidity", async () => {
    const id = "0x" + crypto.createHash('sha256').update("executeRemoveLiquidity1").digest('hex');
    const userBalancePreAUSD = await ausd.balanceOf(receiver.queryEvmAddress());
    const userBalancePreDOT = await dot.balanceOf(receiver.queryEvmAddress());
    const executorBalancePreAUSD = await ausd.balanceOf(executor.queryEvmAddress());
    const executorBalancePreDOT = await dot.balanceOf(executor.queryEvmAddress());
		await ausd.connect(executor as any).approve(escrow.address, amountAUSD)
    await dot.connect(executor as any).approve(escrow.address, amountDOT)
		await expect(escrow.executeRemoveLiquidity(id, receiver.queryEvmAddress(), ausd.address, dot.address, amountAUSD, amountDOT)).to.emit(escrow, "ExecuteRemoveLiquidity")
		await expect(escrow.settleRemoveLiquidity({xtxId: id, shouldCommit: false}, receiver.queryEvmAddress(), ausd.address, dot.address, amountAUSD, amountDOT)).to.emit(escrow, "Revert")
    const userBalancePostAUSD = await ausd.balanceOf(receiver.queryEvmAddress());
    const userBalancePostDOT = await dot.balanceOf(receiver.queryEvmAddress());
    const executorBalancePostAUSD = await ausd.balanceOf(executor.queryEvmAddress());
    const executorBalancePostDOT = await dot.balanceOf(executor.queryEvmAddress());
    expect(userBalancePostAUSD).is.equal(userBalancePreAUSD)
    expect(userBalancePostDOT).is.equal(userBalancePreDOT)
    expect(executorBalancePostAUSD).is.equal(executorBalancePreAUSD)
    expect(executorBalancePostDOT).is.equal(executorBalancePreDOT)
  })
});

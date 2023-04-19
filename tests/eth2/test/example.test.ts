import { expect } from 'chai';
const { ethers } = require("hardhat")

const ONE_ETH = ethers.BigNumber.from("1000000000000000000");

describe('SimpleTransfer', () => {
  it('Transfer Ether from one wallet to another', async () => {
    const [wallet, walletTo] = await ethers.getSigners();

    const value = ONE_ETH;
    const initialRecipientBalance = await walletTo.getBalance(); // The initial balance of the recipient wallet

    const tx = await wallet.sendTransaction({
      to: walletTo.address,
      value: value,
    }); // Send the transaction from the sender wallet to the recipient wallet
    await tx.wait(); // Wait for the transaction to be mined
    
    const finalRecipientBalance = await walletTo.getBalance(); // The final balance of the recipient wallet
    expect(finalRecipientBalance).to.equal(initialRecipientBalance.add(value)); // Check that the recipient wallet balance has increased by the amount sent
  });
});

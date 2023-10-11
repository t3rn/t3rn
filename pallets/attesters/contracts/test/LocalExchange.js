// // Include the helper libraries
// const { ethers } = require('hardhat');
// const { expect } = require('chai');
//
// describe("LocalExchange::Rewards", function() {
//     let contract, USDCContract, owner, user, initialEthBalance, initialUSDCBalance;
//     before(async function() {
//         await require("hardhat").network.provider.request({
//             method: "hardhat_reset",
//             params: [],
//         });
//         // get signers
//         [owner, user, executor] = await ethers.getSigners();
//
//         // deploy contract
//         const LocalExchange = await ethers.getContractFactory("LocalExchange");
//         contract = await LocalExchange.deploy();
//         await contract.deployed();
//
//         // deploy USDC mock contract
//         const ERC20Mock = await ethers.getContractFactory("ERC20Mock");
//         USDCContract = await ERC20Mock.deploy("USD Coin", "USDC");
//         await USDCContract.deployed();
//
//         // mint some USDC for user
//         initialUSDCBalance = ethers.utils.parseUnits('1000', 6); // 1000 USDC
//
//         await USDCContract.mint(user.address, initialUSDCBalance);
//         await USDCContract.mint(executor.address, initialUSDCBalance);
//
//         // save initial balance of user
//         initialEthBalance = await ethers.provider.getBalance(user.address);
//     });
//
//     // Parameters for the order function
//     const destination = "0x03030303"; // arbitrary destination string
//     const asset = "0x05050505"; // arbitrary asset string
//     const amount = ethers.utils.parseUnits('2', 6); // arbitrary amount of 2 ETH
//     const insurance = ethers.utils.parseUnits('3', 6); // arbitrary insurance of 2 ETH
//     const maxRewardETH = ethers.utils.parseUnits('1', 6); // 1 ETH
//     const maxRewardUSDC = ethers.utils.parseUnits('10', 6); // 10 USDC
//     const amountUSDC = ethers.utils.parseUnits('20', 6); // 20 USDC
//     const rewardAssetETH = ethers.constants.AddressZero; // For ETH
//
//     async function getParams() {
//         const [owner, user] = await ethers.getSigners();
//
//         // Parameters for the order function
//         return {
//             destination, // arbitrary destination string
//             asset, // arbitrary asset string
//             targetAccount: user.address, // user's address,
//             executorAccount: user.address, // user's address,
//             amount, // arbitrary amount of 10 ETH
//             amountUSDC, // arbitrary amount of 20 USDC
//             insurance, // arbitrary insurance of 2 ETH
//             maxRewardETH, // 1 ETH
//             maxRewardUSDC, // 100 USDC
//             rewardAssetETH, // For ETH
//             rewardAssetUSDC: USDCContract.address // For USDC
//         };
//     }
//
//
//     it("Should successfully execute local exchange in Eth for Eth", async function() {
//         // approve the contract to spend user's USDC
//         let params = await getParams();
//         const reward = params.maxRewardETH.toNumber();
//         const amount = params.amount.toNumber();
//
//         const localOrderCall = await contract.connect(user).localOrder(
//             ethers.constants.AddressZero,
//             amount,
//             ethers.constants.AddressZero,
//             reward, { value: reward }
//         );
//
//         // check contract Eth balance after localOrder
//         const contractEthBalanceAfterLocalOrder = await ethers.provider.getBalance(contract.address);
//         expect(contractEthBalanceAfterLocalOrder).to.equal(reward);
//
//         const receipt = await localOrderCall.wait();
//         console.log("receipt localOrderCall in ETH -- gas used", receipt.cumulativeGasUsed.toString());
//
//         // Read local order execution ID from event emitted by localOrder
//         const localOrderEvent = receipt.events.find(
//             (e) => e.event === "LocalOrder",
//             "There is no LocalOrder event @ localOrder call"  // Error message
//         );
//         const localOrderId = localOrderEvent.args[0];
//         // expect localOrderId to be a bytes32
//         expect(localOrderId).to.be.a('string');
//         expect(localOrderId).to.have.lengthOf(66);
//         const submissionBlockNumber = receipt.blockNumber;
//
//         // Proceed with order's execution
//         const executionCall = await contract.connect(executor).executeLocalOrder(
//             submissionBlockNumber,
//             user.address,
//             ethers.constants.AddressZero,
//             amount,
//             ethers.constants.AddressZero,
//             reward, { value: amount }
//         );
//
//         const receipt2 = await executionCall.wait();
//         console.log("receipt executionCall in ETH -- gas used", receipt2.cumulativeGasUsed.toString());
//
//         // check contract Eth balance
//         const contractEthBalance = await ethers.provider.getBalance(contract.address);
//         expect(contractEthBalance).to.equal(0);
//     });
//
//     it("Should successfully execute local exchange in USDC for USDC", async function() {
//         // approve the contract to spend user's USDC
//         let params = await getParams();
//         const reward = params.maxRewardUSDC.toNumber();
//         const amount = params.amountUSDC.toNumber();
//
//         await USDCContract.connect(user).approve(contract.address, (reward + amount));
//
//         const localOrderCall = await contract.connect(user).localOrder(
//             USDCContract.address,
//             amount,
//             USDCContract.address,
//             reward
//         );
//
//         const receipt = await localOrderCall.wait();
//         console.log("receipt localOrderCall -- gas used", receipt.cumulativeGasUsed.toString());
//
//         // Read local order execution ID from event emitted by localOrder
//         const localOrderEvent = receipt.events.find(
//             (e) => e.event === "LocalOrder",
//             "There is no LocalOrder event @ localOrder call"  // Error message
//         );
//         const localOrderId = localOrderEvent.args[0];
//         // expect localOrderId to be a bytes32
//         expect(localOrderId).to.be.a('string');
//         expect(localOrderId).to.have.lengthOf(66);
//         // recover submission block number from receipt
//         const submissionBlockNumber = receipt.blockNumber;
//
//         const _approvalCallExecutor = await USDCContract.connect(executor).approve(contract.address, amount);
//
//         // Proceed with order's execution
//         const executionCall = await contract.connect(executor).executeLocalOrder(
//             submissionBlockNumber,
//             user.address,
//             USDCContract.address,
//             amount,
//             USDCContract.address,
//             reward,
//         );
//
//         const receipt2 = await executionCall.wait();
//         console.log("receipt executionCall -- gas used", receipt2.cumulativeGasUsed.toString());
//
//         // // check USDC balance of executor
//         const executorUSDCBalance = await USDCContract.balanceOf(executor.address);
//         expect(executorUSDCBalance).to.equal(initialUSDCBalance.sub(amount).add(reward));
//         // check USDC balance of user
//         const userUSDCBalance = await USDCContract.balanceOf(user.address);
//         expect(userUSDCBalance).to.equal(initialUSDCBalance.sub(reward).add(amount));
//         // check contract USDC balance
//         const contractUSDCBalance = await USDCContract.balanceOf(contract.address);
//         expect(contractUSDCBalance).to.equal(0);
//     });
//
//     it("Should be able to claim back local exchange in Eth for Eth", async function() {
//         // approve the contract to spend user's USDC
//         let params = await getParams();
//         const reward = params.maxRewardETH.toNumber();
//         const amount = params.amount.toNumber();
//
//         const localOrderCall = await contract.connect(user).localOrder(
//             ethers.constants.AddressZero,
//             amount,
//             ethers.constants.AddressZero,
//             reward, { value: reward }
//         );
//
//         // check contract Eth balance after localOrder
//         const contractEthBalanceAfterLocalOrder = await ethers.provider.getBalance(contract.address);
//         expect(contractEthBalanceAfterLocalOrder).to.equal(reward);
//
//         const receipt = await localOrderCall.wait();
//         console.log("receipt localOrderCall in ETH -- gas used", receipt.cumulativeGasUsed.toString());
//
//         // Read local order execution ID from event emitted by localOrder
//         const localOrderEvent = receipt.events.find(
//             (e) => e.event === "LocalOrder",
//             "There is no LocalOrder event @ localOrder call"  // Error message
//         );
//
//         const submissionBlockNumber = receipt.blockNumber;
//
//         // Advance VM to 128 blocks later
//         // Mine 256 blocks
//         await hre.network.provider.send("hardhat_mine", ["0x100"]);
//
//         // Proceed with order's execution
//         const claimCall = await contract.connect(user).claimRefund(
//             submissionBlockNumber,
//             ethers.constants.AddressZero,
//             amount,
//             ethers.constants.AddressZero,
//             reward
//         );
//
//         const receipt2 = await claimCall.wait();
//         console.log("receipt claimCall in ETH -- gas used", receipt2.cumulativeGasUsed.toString());
//
//         // check contract Eth balance
//         const contractEthBalance = await ethers.provider.getBalance(contract.address);
//         expect(contractEthBalance).to.equal(0);
//     });
// });

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract RemoteOrder {

    event OrderCreated(bytes32 indexed id, bytes4 indexed destination, bytes4 asset, address targetAccount, uint256 amount, address rewardAsset, uint256 insurance, uint256 maxReward, uint32 nonce);
    event OrderCommitted(bytes32 indexed id);
    event OrderReverted(bytes32 indexed id);
    event OrderRefundedInERC20(bytes32 indexed id, address indexed asset, uint256 amount);

    struct Order {
        bool exists;
        bytes4 destination;
        bytes4 asset;
        address targetAccount;
        address sender;
        uint256 amount;
        Reward reward;
        OrderStatus status;
    }

    struct Reward {
        address asset;
        uint256 insurance;
        uint256 maxReward;
    }

    enum OrderStatus {
        Accepted,
        Committed,
        Reverted
    }

    mapping(address => uint32) public requestNonce;
    mapping(bytes32 => Order) public orders;
    address owner;

    constructor() {
        owner = msg.sender;
    }

    function generateId(address requester, uint32 nonce) public pure returns (bytes32) {
        return keccak256(abi.encode(requester, nonce));
    }

    /*
     * Before making the order, the function checks that the user has enough balance (of either Ether or the ERC20 token).
     * If everything is okay, it increases the nonce for the user, creates a unique id for the order, saves the order in the mapping,
     * and emits the OrderCreated event.
     */
    function order(
        bytes4 destination,
        bytes4 asset,
        address targetAccount,
        uint256 amount,
        address rewardAsset,
        uint256 insurance,
        uint256 maxReward
    ) public payable {
        require(maxReward > 0, "Max reward must be greater than 0");

        uint32 nonce = requestNonce[msg.sender];
        requestNonce[msg.sender] = nonce + 1;

        bytes32 id = generateId(msg.sender, nonce);

        // Transfer maxReward from user to this contract
        if (rewardAsset == address(0)) {
            require(msg.value >= maxReward, "ETH sent is not enough to cover max reward");
        } else {
            IERC20 token = IERC20(rewardAsset);
            uint256 balance = token.balanceOf(msg.sender);
            require(balance >= maxReward, "Not enough token balance for max reward");
            token.transferFrom(msg.sender, address(this), maxReward);
        }

        orders[id] = Order({
            exists: true,
            destination: destination,
            asset: asset,
            targetAccount: targetAccount,
            sender: msg.sender,
            amount: amount,
            reward: Reward({
            asset: rewardAsset,
            insurance: insurance,
            maxReward: maxReward
        }),
            status: OrderStatus.Accepted
        });

        emit OrderCreated(id, destination, asset, targetAccount, amount, rewardAsset, insurance, maxReward, nonce);
    }

    /*
     * Changes the state of the order from Accepted to Committed.
     * It can only be executed if the order is in the Accepted state.
     */
    function commit(bytes32 id) public {
        require(orders[id].status == OrderStatus.Accepted, "Order must be in Accepted state");
        orders[id].status = OrderStatus.Committed;
        emit OrderCommitted(id);
    }

    /*
     * Refunds the maxReward to the user if the order is in the Accepted state and changes the state to Reverted.
     */
    function revertOrder(bytes32 id) public {
        Order memory order = orders[id];
        require(order.status == OrderStatus.Accepted, "Order must be in Accepted state");

        if (order.reward.asset == address(0)) {
            payable(order.sender).transfer(order.reward.maxReward);
        } else {
            IERC20 token = IERC20(order.reward.asset);
            token.transfer(order.sender, order.reward.maxReward);
            emit OrderRefundedInERC20(id, order.sender, order.reward.maxReward);
        }

        orders[id].status = OrderStatus.Reverted;
        emit OrderReverted(id);
    }

    /*
     * Checks if the id exists in the contract.
     * The function will return true if the order has been Accepted, Committed, or Reverted.
     */
    function isKnownId(bytes32 id) public view returns (bool) {
        return orders[id].exists;
    }
}

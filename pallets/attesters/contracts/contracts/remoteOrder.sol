// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";

contract RemoteOrder {
    using SafeERC20 for IERC20;

    event Test(bytes4 asset);
    event RewardPaid(bytes32 indexed id, address executor, uint256 amount);
    event OrderCreated(
        bytes32 indexed id,
        bytes4 indexed destination,
        bytes4 asset,
        address targetAccount,
        uint256 amount,
        address rewardAsset,
        uint256 insurance,
        uint256 maxReward,
        uint32 nonce
    );
    event OrderCommitted(bytes32 indexed id);
    event OrderReverted(bytes32 indexed id);
    event OrderRefundedInERC20(
        bytes32 indexed id,
        address indexed asset,
        uint256 amount
    );

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

    mapping(bytes32 => Order) public orders;
    mapping(bytes32 => bytes32) public orderPayloads;
    address owner;

    constructor() {
        owner = msg.sender;
    }

    function generateId(
        address requester,
        uint32 nonce
    ) public pure returns (bytes32) {
        return
            keccak256(
                abi.encode(keccak256(abi.encode(requester, nonce)), bytes32(0))
            );
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

        uint32 nonce = uint32(block.number);

        bytes32 id = generateId(msg.sender, nonce);

        // Transfer maxReward from user to this contract
        if (rewardAsset == address(0)) {
            require(
                msg.value >= maxReward,
                "ETH sent is not enough to cover max reward"
            );
        } else {
            IERC20 token = IERC20(rewardAsset);
            uint256 balance = token.balanceOf(msg.sender);
            require(
                balance >= maxReward,
                "Not enough token balance for max reward"
            );
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

        emit OrderCreated(
            id,
            destination,
            asset,
            targetAccount,
            amount,
            rewardAsset,
            insurance,
            maxReward,
            nonce
        );
    }

    /*
     * Changes the state of the order from Accepted to Committed.
     * It can only be executed if the order is in the Accepted state.
     */
    function commit(bytes32 id) public {
        require(
            orders[id].status == OrderStatus.Accepted,
            "Order must be in Accepted state"
        );
        // TODO only guardian can do
        orders[id].status = OrderStatus.Committed;
        emit OrderCommitted(id);
    }

    /*
     * Refunds the maxReward to the user if the order is in the Accepted state and changes the state to Reverted.
     */
    function revertOrder(bytes32 id) public {
        // TODO guardian should check continuosly for not commited orders and send money
        Order memory order = orders[id];
        require(
            order.status == OrderStatus.Accepted,
            "Order must be in Accepted state"
        );
        orders[id].status = OrderStatus.Reverted;

        if (order.reward.asset == address(0)) {
            Address.sendValue(payable(order.sender), order.reward.maxReward);
        } else {
            IERC20 token = IERC20(order.reward.asset);
            token.safeTransfer(order.sender, order.reward.maxReward);
            emit OrderRefundedInERC20(id, order.sender, order.reward.maxReward);
        }

        emit OrderReverted(id);
    }

    /*
     * Checks if the id exists in the contract.
     * The function will return true if the order has been Accepted, Committed, or Reverted.
     */
    function isKnownId(bytes32 id) public view returns (bool) {
        return orders[id].exists;
    }

    function withdrawReward(
        bytes32 id,
        address payable recipient,
        uint256 amount
    ) public {
        require(
            msg.sender == 0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC,
            "Only Guardian can release rewards"
        );
        require(
            address(this).balance >= amount,
            "Not enough Ether in the contract"
        );
        // TODO: withdraw must match what Guardian is sending
        bool success = recipient.send(amount);
        require(success, "Ether transfer failed");

        emit RewardPaid(id, recipient, amount);
    }
}

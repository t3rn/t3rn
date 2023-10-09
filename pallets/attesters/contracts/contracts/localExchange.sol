// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";


struct LocalOrderEntry {
    address sender;
    address nonce;
    address asset;
    uint256 amount;
    address rewardAsset;
    uint256 rewardAmount;
}


contract LocalExchange is Ownable {
    using SafeERC20 for IERC20;

    mapping(address => uint32) public requestNonce;
    mapping(bytes32 => bool) public localOrders;

    event LocalOrder(
        bytes32 indexed id,
        address indexed token,
        uint256 indexed amount,
        address rewardToken,
        uint256 reward
    );

    event OrderExecuted(
        address indexed user,
        address indexed executor,
        address indexed token,
        uint256 amount
    );

    modifier ensureBalanceAndAllowance(address token, address user, uint256 amount) {
        // Ensure the user has enough balance and allowance for the token.
        if (token == address(0)) {
            require(
                user.balance >= amount,
                "Insufficient user balance of native"
            );
            require(
                msg.value == amount,
                "Mismatched deposit execution amount of native"
            );
        } else {
            require(
                IERC20(token).balanceOf(user) >= amount,
                "Insufficient user balance"
            );
            require(
                IERC20(token).allowance(user, address(this)) >= amount,
                "Insufficient user allowance"
            );
        }
        _;
    }

    modifier ensureAllowance(address token, address user, uint256 amount) {
        // Ensure the user has enough balance and allowance for the token.
        if (token != address(0)) {
            require(
                IERC20(token).balanceOf(user) >= amount,
                "Insufficient user balance"
            );
            require(
                IERC20(token).allowance(user, address(this)) >= amount,
                "Insufficient user allowance"
            );
        }
        _;
    }

    function localOrder(
         address token,
         uint256 amount,
         address rewardToken,
         uint256 reward
    ) payable external ensureBalanceAndAllowance(rewardToken, msg.sender, reward) {
        // Create a unique ID for the order
        bytes32 id = keccak256(abi.encodePacked(msg.sender, block.number));
        // Rehash the ID with order values to check at execution
        bytes32 local_order_id = keccak256(abi.encodePacked(id, token, amount, rewardToken, reward));
        // Store the order ID if it doesn't exist -- or revert if it does allowing for 1 order per Block
        require(!localOrders[local_order_id], "Order already exists");
        localOrders[local_order_id] = true;
        // Emit the order
        emit LocalOrder(id, token, amount, rewardToken, reward);
    }

    function executeLocalOrder(
        uint256 orderSubmissionBlockNumber,
        address payable user,
        address token,
        uint256 amount,
        address rewardToken,
        uint256 reward
    ) payable external ensureBalanceAndAllowance(token, msg.sender, amount) ensureAllowance(rewardToken, user, reward) {
        // Derive order ID from the order submission block number and the user's address
        bytes32 id = keccak256(abi.encodePacked(user, orderSubmissionBlockNumber));
        // Rehash the ID with order values to check at execution
        bytes32 local_order_id = keccak256(abi.encodePacked(id, token, amount, rewardToken, reward));
        // Check if the order exists
        require(localOrders[local_order_id], "Order does not exist");

        // Ensure the order has not been executed after timeout
        require(block.number < orderSubmissionBlockNumber + 128, "Order has timed out");

        if (amount > 0) {
            // If the reward token is ETH
            if (token == address(0)) {
                payable(user).transfer(amount);
            } else { // If the reward token is an ERC-20 token
                IERC20(token).safeTransferFrom(msg.sender, user, amount);
            }
        }

        if (reward > 0 && token != address(0)) {
            IERC20(rewardToken).safeTransferFrom(user, msg.sender, reward);
        } else if (reward > 0 && token == address(0)) {
            payable(msg.sender).transfer(reward);
        }

        // Mark the order as executed
        localOrders[local_order_id] = false;
        emit OrderExecuted(user, msg.sender, token, amount);
    }

    function claimRefund(
        uint256 orderSubmissionBlockNumber,
        address token,
        uint256 amount,
        address rewardToken,
        uint256 reward
    ) external payable {
        address payable user = payable(msg.sender);
        // Derive order ID from the order submission block number and the user's address
        bytes32 id = keccak256(abi.encodePacked(user, orderSubmissionBlockNumber));
        // Rehash the ID with order values to check at execution
        bytes32 local_order_id = keccak256(abi.encodePacked(id, token, amount, rewardToken, reward));
        // Check if the order exists
        require(localOrders[local_order_id], "Order does not exist or has already been executed|claimed");
        // Ensure claim is made after timeout
        require(block.number >= orderSubmissionBlockNumber + 128, "Order has not timed out");
        // Mark the order as executed
        localOrders[local_order_id] = false;
        // If the reward token is ETH
        if (rewardToken == address(0)) {
            payable(user).transfer(reward);
        } else { // If the reward token is an ERC-20 token -- the amount was not transferred to the contract, only allowance was given. So no need to transfer back. Just reset allowance.
            IERC20(rewardToken).safeApprove(address(this), 0);
        }
    }

    // Allow contract to receive Ether
    receive() external payable {}

    // Allow the owner to withdraw Ether
    function withdrawEther(address payable to, uint256 amount) external onlyOwner {
        to.transfer(amount);
    }
}

pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";


interface It3rnVault {

    function deposit(address asset, uint256 amount) external payable;

    function withdraw(address asset, uint256 amount, address to) external payable;
}

contract EscrowGMP {
    using SafeERC20 for IERC20;

    It3rnVault private vault;

    event ExecutedLocally(bytes32 id, address sender);
    enum ActionType { LocalPayment, RemotePayment, EscrowCall }

    struct LocalPayment {
        address payable sender;
        uint32 nonce;
        address asset;
        address rewardAsset;
        uint256 amount;
        uint256 rewardAmount;
    }

    struct RemotePayment { bool executed; }

    struct Call {
        bytes callData;
        address destination;
    }

    struct EscrowCall {
        Call onCommit;
        Call onRevert;
    }

    // Note: RemotePayment only stores a bool so we might not need a full struct

    mapping(bytes32 => uint256) public localPayments;
    mapping(bytes32 => EscrowCall) public escrowCalls;
    mapping(bytes32 => bool) public remotePayments;

    constructor(address _vault) {
        vault = It3rnVault(_vault);
    }

    function storeLocalOrderPayload(bytes calldata data) external returns (bytes32, LocalPayment memory) {
        bytes32 id = keccak256(data);
        // Decode data into appropriate parameters
        (LocalPayment memory payment) = abi.decode(data, (LocalPayment));
        // Store the payment if it hasn't been stored already
        localPayments[id] = block.number;
        return (id, payment);
    }

    function storeLocalOrderPayloadCallData(address payable sender, uint32 nonce, bytes calldata data) external returns (bytes32, LocalPayment memory) {
        // Decode data into appropriate parameters
        (address asset, address rewardAsset, uint256 amount, uint256 rewardAmount) = abi.decode(data, (address, address, uint256, uint256));

        bytes32 id = keccak256(abi.encode(sender, nonce, asset, rewardAsset, amount, rewardAmount));

        // Store the payment if it hasn't been stored already
        localPayments[id] = block.number;
        return (id, (LocalPayment(sender, nonce, asset, rewardAsset, amount, rewardAmount)));
    }

    function storeId(bytes calldata data, ActionType actionType) external returns (bytes32) {
        if (actionType == ActionType.LocalPayment) {
            (bytes32 id, LocalPayment memory localPayment) = this.storeLocalOrderPayload(data);
            return id;
        } else if (actionType == ActionType.RemotePayment) {
            // Decode data into appropriate parameters
            bytes32 id = abi.decode(data, (bytes32));
            // Process RemotePayment
            remotePayments[id] = true;
            return id;

        } else if (actionType == ActionType.EscrowCall) {
            (bytes32 id, EscrowCall memory call) = abi.decode(data, (bytes32, EscrowCall));
            // Decode data into Call struct
            // Store the callData and destination
            escrowCalls[id] = call;
            return id;
        }
        return 0;
    }

    function executeLocally(bytes calldata data) external payable {
        // Decode data into appropriate parameters
        (bytes32 id, LocalPayment memory payment) = abi.decode(data, (bytes32, LocalPayment));
        // Recover local payment
        uint256 recoveredBlockForId = localPayments[id];
        require(recoveredBlockForId != 0, "Local payment not found");
        // Check if the payment has been executed
        require(recoveredBlockForId < block.number + 128, "Local order has timed out");
        // Execute the payment
        if (payment.rewardAsset == address(0)) {
            msg.sender.call{value: payment.amount}("");
        } else {
            IERC20(payment.rewardAsset).safeTransferFrom(msg.sender, address(payment.sender), payment.amount);
        }
        // Execute the reward
        if (payment.rewardAsset == address(0)) {
            msg.sender.call{value: payment.rewardAmount}("");
        } else {
            IERC20(payment.rewardAsset).safeTransferFrom(msg.sender, address(payment.sender), payment.rewardAmount);
        }
        localPayments[id] = 1;

        emit ExecutedLocally(id, msg.sender);
    }

    function commit(bytes32 id, ActionType actionType) external {
        if (actionType == ActionType.LocalPayment) {
            // Handle LocalPayment commit
            // Withdraw from vault and transfer assets
        } else if (actionType == ActionType.RemotePayment) {
            // Handle RemotePayment commit
        } else if (actionType == ActionType.EscrowCall) {
            // Handle Call commit
            // Extract callData and destination, then delegateCall
        }
    }

    function xRevert(bytes32 id, ActionType actionType) external {
        if (actionType == ActionType.LocalPayment) {
            // Handle LocalPayment revert
            // Withdraw from vault and transfer assets back
        } else if (actionType == ActionType.RemotePayment) {
            // Handle RemotePayment revert
        } else if (actionType == ActionType.EscrowCall) {
            // Handle Call revert
            // Extract revert callData and destination, then delegateCall
        }
    }
}
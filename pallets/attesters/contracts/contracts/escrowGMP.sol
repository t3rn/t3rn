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
    address private attesters;

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

    struct RemotePayment { address payable executor; bytes32 payloadHash; }

    struct Call {
        bytes callData;
        address destination;
    }

    struct EscrowCall {
        Call onCommit;
        Call onRevert;
    }

    mapping(bytes32 => uint256) public localPayments;
    mapping(bytes32 => EscrowCall) public escrowCalls;
    mapping(bytes32 => bool) public remotePayments;
    mapping(bytes32 => RemotePayment) public remotePaymentsPayload;
    mapping(bytes32 => bytes32) public remotePaymentsPayloadHash;
    mapping(bytes32 => address) public remotePaymentsPayloadExecutor;

    constructor(address _vault, address _attesters) {
        vault = It3rnVault(_vault);
        attesters = _attesters;
    }

    modifier onlyAttesters() {
        require(msg.sender == attesters, "Only Attesters can call this function");
        _;
    }

    function storeLocalOrderPayload(bytes calldata data) external returns (bytes32, LocalPayment memory) {
        bytes32 id = keccak256(data);
        // Decode data into appropriate parameters
        (LocalPayment memory payment) = abi.decode(data, (LocalPayment));
        // Store the payment if it hasn't been stored already
        localPayments[id] = block.number;
        return (id, payment);
    }

    function storeRemoteOrderPayload(bytes32 sfxId, bytes32 payloadHash) external returns (bool) {
        // Store the payment payload (hash of the payload)
        remotePaymentsPayloadHash[sfxId] = payloadHash;
        return (true);
    }

    function commitRemoteExecutorPayload(bytes32 sfxId, address executor) onlyAttesters external returns (bool) {
        // Update the payment payload (hash of the payload)
        bytes32 currentHash = remotePaymentsPayloadHash[sfxId];
        require(currentHash != 0, "Payload not found");
        bytes32 newHash = keccak256(abi.encode(currentHash, executor));
        remotePaymentsPayloadHash[sfxId] = newHash;
        return (true);
    }

    function withdrawFromVault(bytes32 sfxId, address rewardAsset, uint256 maxReward) payable external {
        bytes32 paymentPayloadHash = keccak256(abi.encode(rewardAsset, maxReward));
        bytes32 calculatedWithdrawHash = keccak256(abi.encode(paymentPayloadHash, msg.sender));
        bytes32 paymentHash = remotePaymentsPayloadHash[sfxId];
        require(paymentHash == calculatedWithdrawHash, "Payload for payment not matching");
        remotePaymentsPayloadHash[sfxId] = bytes32(0);
        vault.withdraw{value: msg.value}(rewardAsset, maxReward, msg.sender);
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
        require(recoveredBlockForId != 1, "Local payment already executed");
        // Check if the payment has been executed
        require(recoveredBlockForId < block.number + 128, "Local order has timed out");
        // Execute the payment
        if (payment.asset == address(0)) {
            require(msg.value == payment.amount, "Mismatched deposit amount");
            payable(payment.sender).transfer(payment.amount);
        } else {
            IERC20(payment.asset).safeTransferFrom(msg.sender, address(payment.sender), payment.amount);
        }

        vault.withdraw{value: msg.value}(payment.rewardAsset, payment.rewardAmount, msg.sender);

        localPayments[id] = 1;

        emit ExecutedLocally(id, msg.sender);
    }

    function commitEscrowCall(bytes32 id) external onlyAttesters returns (bool) {
        // Handle Call commit
        // Extract callData and destination, then delegateCall
        Call memory call = escrowCalls[id].onCommit;
        (bool success, ) = call.destination.delegatecall(call.callData);
        return success;
    }

    function revertEscrowCall(bytes32 id) onlyAttesters external {
        // Handle Call revert
        // Extract callData and destination, then delegateCall
        Call memory call = escrowCalls[id].onRevert;
        (bool success, ) = call.destination.delegatecall(call.callData);
        require(success, "Revert failed");
    }
}
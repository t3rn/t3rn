pragma solidity ^0.8.0;

import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "./t3rnVault.sol";

contract EscrowGMP {

    t3rnVault private vault;
    address private attesters;
    address private owner;

    enum ActionType { RemotePayment, EscrowCall }

    struct RemotePayment { address payable executor; bytes32 payloadHash; }

    struct Call {
        bytes callData;
        address destination;
    }

    struct EscrowCall {
        Call onCommit;
        Call onRevert;
    }

    mapping(bytes32 => EscrowCall) public escrowCalls;
    mapping(bytes32 => bool) public remotePayments;
    mapping(bytes32 => bytes32) public remotePaymentsPayloadHash;

    constructor(t3rnVault _vault) {
        vault = _vault;
        owner = msg.sender;
    }

    function assignAttesters(address _attesters) external onlyOwner {
        attesters = _attesters;
    }

    modifier onlyOwner() {
        require(msg.sender == owner, "Only owner can call this function");
        _;
    }

    modifier onlyAttesters() {
        require(msg.sender == attesters, "Only Attesters can call this function");
        _;
    }

    function storeRemoteOrderPayload(bytes32 sfxId, bytes32 payloadHash) external {
        // Check if the payload is already stored and return false if it is
        if (remotePaymentsPayloadHash[sfxId] == 0) {
            // Store the payment payload (hash of the payload)
            remotePaymentsPayloadHash[sfxId] = payloadHash;
        }
    }

    function commitRemoteBeneficiaryPayload(bytes32 sfxId, address beneficiary) onlyAttesters external returns (bool) {
        // Update the payment payload (hash of the payload)
        bytes32 currentHash = remotePaymentsPayloadHash[sfxId];
        require(currentHash != 0, "Payload not found");
        bytes32 newHash = keccak256(abi.encode(currentHash, beneficiary));
        remotePaymentsPayloadHash[sfxId] = newHash;
        return (true);
    }

    function revertRemoteOrderPayload(bytes32 sfxId) onlyAttesters external returns (bool) {
        // Update the payment payload (hash of the payload)
        bytes32 currentHash = remotePaymentsPayloadHash[sfxId];
        require(currentHash != 0, "Payload not found");
        bytes32 newHash = keccak256(abi.encode(currentHash, address(0)));
        remotePaymentsPayloadHash[sfxId] = newHash;
        return (true);
    }

    function withdrawFromVault(bytes32 sfxId, address rewardAsset, uint256 amount) payable external {
        bytes32 paymentPayloadHash = keccak256(abi.encode(rewardAsset, amount));
        bytes32 calculatedWithdrawHash = keccak256(abi.encode(paymentPayloadHash, msg.sender));
        bytes32 calculatedRefundHash = keccak256(abi.encode(paymentPayloadHash, address(0)));
        bytes32 paymentHash = remotePaymentsPayloadHash[sfxId];
        require(paymentHash == calculatedWithdrawHash || paymentHash == calculatedRefundHash, "Payload for payment not matching");
        remotePaymentsPayloadHash[sfxId] = bytes32(0);
        vault.withdraw{value: msg.value}(rewardAsset, amount, msg.sender);
    }

    function withdrawFromVaultSkipGMPChecks(address asset, uint256 amount, address beneficiary) onlyAttesters payable external {
        vault.withdraw{value: amount}(asset, amount, beneficiary);
    }

    function storeEscrowCallOrder(bytes calldata data) external returns (bytes32) {
        // Decode data into Call struct
        (bytes32 id, EscrowCall memory call) = abi.decode(data, (bytes32, EscrowCall));
        // Store the callData and destination
        escrowCalls[id] = call;
        return id;
    }

    // Handle Call commit
    function commitEscrowCall(bytes32 id) external onlyAttesters returns (bool) {
        // Extract callData and destination, then delegateCall
        Call memory call = escrowCalls[id].onCommit;
        (bool success, ) = call.destination.delegatecall(call.callData);
        return success;
    }

    // Handle Call Revert
    function revertEscrowCall(bytes32 id) onlyAttesters external returns (bool) {
        // Extract callData and destination, then delegateCall
        Call memory call = escrowCalls[id].onRevert;
        (bool success, ) = call.destination.delegatecall(call.callData);
        return success;
    }
}
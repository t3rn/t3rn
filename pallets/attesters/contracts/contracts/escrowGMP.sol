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

    enum ActionType { LocalPayment, RemotePayment, EscrowCall }

    struct LocalPayment {
        address sender;
        address asset;
        uint256 amount;
        uint256 blockNumber;
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

    mapping(bytes32 => LocalPayment) public localPayments;
    mapping(bytes32 => EscrowCall) public escrowCalls;
    mapping(bytes32 => bool) public remotePayments;

    constructor(address _vault) {
        vault = It3rnVault(_vault);
    }

    function execute(bytes calldata data, ActionType actionType) external payable {
        if (actionType == ActionType.LocalPayment) {
            // Decode data into appropriate parameters
            (bytes32 id, LocalPayment memory payment) = abi.decode(data, (bytes32, LocalPayment));
            // Store the payment
            localPayments[id] = payment;
        } else if (actionType == ActionType.RemotePayment) {
            // Decode data into appropriate parameters
            bytes32 id = abi.decode(data, (bytes32));
            // Process RemotePayment
            remotePayments[id] = true;
        } else if (actionType == ActionType.EscrowCall) {
            (bytes32 id, EscrowCall memory call) = abi.decode(data, (bytes32, EscrowCall));
            // Decode data into Call struct
            // Store the callData and destination
            escrowCalls[id] = call;
        }
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
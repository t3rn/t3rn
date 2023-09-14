// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";

    struct LocalPayment {
        address sender;
        address nonce;
        address asset;
        address rewardAsset;
        uint256 amount;
        uint256 rewardAmount;
    }

    enum ActionType { LocalPayment, RemotePayment, EscrowCall }

interface IEscrowGMP {

    function storeLocalOrderPayload(bytes calldata data) external returns (bytes32, LocalPayment calldata);
    function storeLocalOrderPayloadCallData(address sender, uint32 nonce, bytes calldata data) external returns (bytes32, LocalPayment memory);

    function storeId(bytes calldata data, ActionType actionType) external returns (bytes32);
    function execute(bytes calldata data, ActionType actionType) external payable;

    function commit(bytes32 id, ActionType actionType) external;

    function xRevert(bytes32 id, ActionType actionType) external;
}

contract RemoteOrder is ReentrancyGuard  {

    using SafeERC20 for IERC20;

    IEscrowGMP private escrowGMP;

    event RemoteOrderCreated(bytes32 id, bytes4 destination, bytes4 asset, bytes32 targetAccount, uint256 amount, address rewardAsset, uint256 insurance, uint256 maxReward, uint32 nonce, address sender);
    event RemoteOrderIndexedCreated(bytes32 indexed id, uint32 indexed nonce, address indexed sender, bytes input);
    event LocalOrderCreated(bytes32 id, address sender, uint32 nonce, address asset, uint256 amount, address rewardAsset, uint256 maxReward);

    mapping(address => uint32) public requestNonce;

    constructor(address _escrowGMPAddress) {
        escrowGMP = IEscrowGMP(_escrowGMPAddress);
    }

    function generateId(address requester, uint32 nonce) public pure returns (bytes32) {
        return keccak256(abi.encode(keccak256(abi.encode(requester, nonce)), bytes32(0)));
    }

    /*
     * Before making the order, the function checks that the user has enough balance (of either Ether or the ERC20 token).
     * If everything is okay, it increases the nonce for the user, creates a unique id for the order, saves the order in the mapping,
     * and emits the OrderCreated event.
     */
    function remoteOrder(
        bytes calldata input
    ) public payable {

        uint32 nonce = requestNonce[msg.sender];
        requestNonce[msg.sender] = nonce + 1;

        bytes32 id = generateId(msg.sender, nonce);

        (bytes4 destination, bytes4 asset, bytes32 targetAccount, uint256 amount, address rewardAsset, uint256 insurance, uint256 maxReward) = abi.decode(input, (bytes4, bytes4, bytes32, uint256, address, uint256, uint256));
        // Accept temporary ownership of assets
        if (rewardAsset == address(0)) {
            require(msg.value == amount, "Mismatched deposit amount");
        } else {
            IERC20(rewardAsset).safeTransferFrom(msg.sender, address(this), amount);
        }

        escrowGMP.execute(abi.encode(id), ActionType.RemotePayment); // store token payment

        // emit RemoteOrderCreated(id, destination, asset, targetAccount, amount, rewardAsset, insurance, maxReward, nonce, msg.sender);
        emit RemoteOrderIndexedCreated(id, nonce, msg.sender, input);
    }

    /*
     * Before making the order, the function checks that the user has enough balance (of either Ether or the ERC20 token).
     * If everything is okay, it increases the nonce for the user, creates a unique id for the order, saves the order in the mapping,
     * and emits the OrderCreated event.
     */
    function localOrderCall(
        bytes calldata input
    ) public payable {

        uint32 nonce = requestNonce[msg.sender];
        requestNonce[msg.sender] = nonce + 1;

        (bytes32 id, LocalPayment memory payment) = escrowGMP.storeLocalOrderPayloadCallData(msg.sender, nonce, input); // store token payment
        emit LocalOrderCreated(id, payment.sender, nonce, payment.asset, payment.amount, payment.rewardAsset, payment.rewardAmount);

         // Accept temporary ownership of assets
         if (payment.rewardAsset == address(0)) {
             require(msg.value == payment.amount, "Mismatched deposit amount");
             msg.sender.call{value: payment.amount}("");
         } else {
             IERC20(payment.rewardAsset).safeTransferFrom(msg.sender, address(this), payment.amount);
         }
    }

    /*
     * Before making the order, the function checks that the user has enough balance (of either Ether or the ERC20 token).
     * If everything is okay, it increases the nonce for the user, creates a unique id for the order, saves the order in the mapping,
     * and emits the OrderCreated event.
     */
    function localOrder(
        address asset,
        address rewardAsset,
        uint256 amount,
        uint256 rewardAmount
    ) public payable {

        uint32 nonce = requestNonce[msg.sender];
        requestNonce[msg.sender] = nonce + 1;

        (bytes32 id, LocalPayment memory payment) = escrowGMP.storeLocalOrderPayload(abi.encode(msg.sender, nonce, asset, rewardAsset, amount, rewardAmount)); // store token payment

        // Accept temporary ownership of assets
        if (payment.rewardAsset == address(0)) {
            require(msg.value == amount, "Mismatched deposit amount");
            msg.sender.call{value: payment.amount}("");
        } else {
            IERC20(rewardAsset).safeTransferFrom(msg.sender, address(this), amount);
        }

        emit LocalOrderCreated(id, payment.sender, nonce, payment.asset, payment.amount, payment.rewardAsset, payment.rewardAmount);
    }
}

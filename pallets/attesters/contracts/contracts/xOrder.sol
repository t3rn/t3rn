// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";

struct LocalPayment {
    address payable sender;
    address nonce;
    address asset;
    address rewardAsset;
    uint256 amount;
    uint256 rewardAmount;
}

enum ActionType { LocalPayment, RemotePayment, EscrowCall }
interface It3rnVault {

    function deposit(address asset, uint256 amount) external payable;

    function withdraw(address asset, uint256 amount, address to) external payable;
}

interface IEscrowGMP {
    function storeLocalOrderPayload(bytes calldata data) external returns (bytes32, LocalPayment calldata);
    function storeLocalOrderPayloadCallData(address sender, uint32 nonce, bytes calldata data) external returns (bytes32, LocalPayment memory);
    function storeRemoteOrderPayload(bytes32 sfxId, bytes32 payloadHash) external returns (bool);
}

contract RemoteOrder {

    using SafeERC20 for IERC20;

    IEscrowGMP private escrowGMP;
    It3rnVault private vault;
    address private attesters;

    event RemoteOrderCreated(bytes32 id, bytes4 destination, bytes4 asset, bytes32 targetAccount, uint256 amount, address rewardAsset, uint256 insurance, uint256 maxReward, uint32 nonce, address sender);
    event RemoteOrderIndexedCreated(bytes32 indexed id, uint32 indexed nonce, address indexed sender, bytes input);
    event LocalOrderCreated(bytes32 id, address sender, uint32 nonce, address asset, uint256 amount, address rewardAsset, uint256 maxReward);

    mapping(address => uint32) public requestNonce;

    constructor(address _escrowGMPAddress, address _vault) {
        escrowGMP = IEscrowGMP(_escrowGMPAddress);
        vault = It3rnVault(_vault);
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
            require(msg.value == maxReward, "Mismatched deposit amount");
            payable(address(vault)).transfer(maxReward);
        } else {
            IERC20(rewardAsset).safeTransferFrom(msg.sender, address(vault), maxReward);
        }

        escrowGMP.storeRemoteOrderPayload(id, keccak256(abi.encode(rewardAsset, maxReward)));

        emit RemoteOrderIndexedCreated(id, nonce, msg.sender, input);
    }


    /*
    * Before making the order, the function checks that the user has enough balance (of either Ether or the ERC20 token).
    * If everything is okay, it increases the nonce for the user, creates a unique id for the order, saves the order in the mapping,
    * and emits the OrderCreated event.
    */
    function remoteOrderDecoded(
        bytes4 destination,
        bytes4 asset,
        bytes32 targetAccount,
        uint256 amount,
        address rewardAsset,
        uint256 insurance,
        uint256 maxReward
    ) public payable {

        uint32 nonce = requestNonce[msg.sender];
        requestNonce[msg.sender] = nonce + 1;

        bytes32 id = generateId(msg.sender, nonce);

        // Accept temporary ownership of assets
        if (rewardAsset == address(0)) {
            require(msg.value == amount, "Mismatched deposit amount");
            payable(address(vault)).transfer(maxReward);
        } else {
            IERC20(rewardAsset).safeTransferFrom(msg.sender, address(vault), maxReward);
        }

        escrowGMP.storeRemoteOrderPayload(id, keccak256(abi.encode(rewardAsset, maxReward)));

        emit RemoteOrderIndexedCreated(id, nonce, msg.sender, abi.encode(destination, asset, targetAccount, amount, rewardAsset, insurance, maxReward));
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
             require(msg.value == payment.rewardAmount, "Mismatched deposit amount");
             payable(address(vault)).transfer(payment.rewardAmount);
         } else {
             IERC20(payment.rewardAsset).safeTransferFrom(msg.sender, address(vault), payment.rewardAmount);
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

        // Accept temporary ownership of assets to EscrowGMP
        if (payment.rewardAsset == address(0)) {
            require(msg.value == rewardAmount, "Mismatched deposit amount");
            payable(address(vault)).transfer(rewardAmount);
        } else {
            IERC20(rewardAsset).safeTransferFrom(msg.sender, address(vault), rewardAmount);
        }

        emit LocalOrderCreated(id, payment.sender, nonce, payment.asset, payment.amount, payment.rewardAsset, payment.rewardAmount);
    }
}

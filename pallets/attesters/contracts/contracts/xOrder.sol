// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "./t3rnVault.sol";
import "./escrowGMP.sol";

enum ActionType { RemotePayment, EscrowCall }

contract XOrder {

    using SafeERC20 for IERC20;

    EscrowGMP public escrowGMP;
    address payable private vault;
    address public owner;
    address private attesters;
    mapping(address => uint32) public supportedBridgeAssetsHereToThere;

    event RemoteOrderCreated(bytes32 indexed id, uint32 indexed nonce, address indexed sender, bytes input);

    constructor(EscrowGMP _escrowGMPAddress, address payable _vault) {
        escrowGMP = _escrowGMPAddress;
        vault = _vault;
        owner = msg.sender;
    }

    modifier onlyOwner() {
        require(msg.sender == owner, "Only owner can call this function");
        _;
    }

    function generateId(address requester, uint32 nonce) public pure returns (bytes32) {
        return keccak256(abi.encode(keccak256(abi.encode(requester, nonce)), bytes32(0)));
    }

    function addSupportedBridgeAsset(address assetHere, uint32 assetThere) public onlyOwner {
        supportedBridgeAssetsHereToThere[assetHere] = assetThere;
    }

    /*
     * Before making the order, the function checks that the user has enough balance (of either Ether or the ERC20 token).
     * If everything is okay, it increases the nonce for the user, creates a unique id for the order, saves the order in the mapping,
     * and emits the OrderCreated event.
     */
    function orderMemoryData(
        bytes memory input
    ) public payable {
        uint32 nonce = uint32(block.number);
        bytes32 id = generateId(msg.sender, nonce);

        (bytes4 destination, uint32 asset, bytes32 targetAccount, uint256 amount, address rewardAsset, uint256 insurance, uint256 maxReward) = abi.decode(input, (bytes4, uint32, bytes32, uint256, address, uint256, uint256));
        // Accept temporary ownership of assets
        if (rewardAsset == address(0)) {
            require(msg.value == maxReward, "Mismatched deposit amount");
            bool sent = vault.send(msg.value);
            require(sent, "Failed to send Ether to vault");
        } else {
            IERC20(rewardAsset).safeTransferFrom(msg.sender, address(vault), maxReward);
        }
        require(escrowGMP.storeRemoteOrderPayload(id, keccak256(abi.encode(rewardAsset, maxReward))), "Payload already stored");

        emit RemoteOrderCreated(id, nonce, msg.sender, input);
    }

    /*
     * Before making the order, the function checks that the user has enough balance (of either Ether or the ERC20 token).
     * If everything is okay, it increases the nonce for the user, creates a unique id for the order, saves the order in the mapping,
     * and emits the OrderCreated event.
     */
    function remoteOrder(
        bytes calldata input
    ) public payable {
        orderMemoryData(input);
    }

    function remoteBridgeAsset(
        address assetHere,
        bytes32 targetAccount,
        uint256 amount,
        uint256 maxRewardSubtractedFromAmount
    ) public payable {

        uint32 assetThere = supportedBridgeAssetsHereToThere[assetHere];
        require(assetThere != 0, "Unsupported bridge asset");

        bytes memory input = abi.encode(bytes4(0x03030303), assetThere, targetAccount, amount, assetHere, uint256(0) , maxRewardSubtractedFromAmount);

        orderMemoryData(input);
    }

    function claimPayoutOrRefund(
        bytes32 id,
        address rewardAsset,
        uint256 maxReward
    ) public payable {
        escrowGMP.withdrawFromVault(id, rewardAsset, maxReward);
    }

    struct Payout {
        bytes32 id;
        address rewardAsset;
        uint256 maxReward;
    }

    function claimPayoutOrRefundBatch(
        Payout[] memory payouts
    ) public payable {
        // Instead of mapping, we use two arrays:
        address[] memory rewardAssets = new address[](payouts.length);
        uint256[] memory rewardAmounts = new uint256[](payouts.length);

        uint256 rewardAssetCount = 0;
        for (uint256 i = 0; i < payouts.length; i++) {
            bool found = false;
            for (uint256 j = 0; j < rewardAssetCount; j++) {
                if (rewardAssets[j] == payouts[i].rewardAsset) {
                    rewardAmounts[j] += payouts[i].maxReward;
                    found = true;
                    break;
                }
            }
            if (!found) {
                rewardAssets[rewardAssetCount] = payouts[i].rewardAsset;
                rewardAmounts[rewardAssetCount] = payouts[i].maxReward;
                rewardAssetCount++;
            }
        }

        for (uint256 i = 0; i < rewardAssetCount; i++) {
            escrowGMP.withdrawFromVault(payouts[i].id, rewardAssets[i], rewardAmounts[i]);
        }
    }

    /*
    * Before making the order, the function checks that the user has enough balance (of either Ether or the ERC20 token).
    * If everything is okay, it increases the nonce for the user, creates a unique id for the order, saves the order in the mapping,
    * and emits the OrderCreated event.
    */
    function remoteOrderDecoded(
        bytes4 destination,
        uint32 asset,
        bytes32 targetAccount,
        uint256 amount,
        address rewardAsset,
        uint256 insurance,
        uint256 maxReward
    ) public payable {
        bytes memory input = abi.encodePacked(destination, asset, targetAccount, amount, rewardAsset, insurance, maxReward);
        orderMemoryData(input);
    }
}

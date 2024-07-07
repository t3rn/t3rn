
import "../../../contracts/t3Portal.sol";
import "../../../contracts/t3CircuitOrder.sol";

contract MultiCrossChainTransfer {

    t3Portal private PORTAL;
    t3CircuitOrder private CIRCUIT;

    constructor(address portalAddress, address circuitAddress) {
        PORTAL = t3Portal(portalAddress);
        CIRCUIT = t3CircuitOrder(circuitAddress);
    }

    event TransferSentToTarget(bytes4 indexed targetChain, address indexed sender, bytes32 receiver, uint256 amount, address asset);

    function multiOrderToArbitrumAndPolkadot(
        uint32 asset,
        bytes32 targetAccountPolkadot,
        bytes32 targetAccountArbitrum,
        uint256 amount,
        address rewardAsset,
        uint256 maxReward
    ) public {
        CIRCUIT.order(bytes4("arbt"), asset, targetAccountArbitrum, amount, rewardAsset, 0, maxReward);
        CIRCUIT.order(bytes4("pdot"), asset, targetAccountPolkadot, amount, rewardAsset, 0, maxReward);

        emit TransferSentToTarget(bytes4("arbt"), msg.sender, targetAccountArbitrum, amount, rewardAsset);
        emit TransferSentToTarget(bytes4("pdot"), msg.sender, targetAccountPolkadot, amount, rewardAsset);
    }
}






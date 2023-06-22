// SPDX-License-Identifier: GPL-3.0
pragma solidity >=0.7.0 <0.9.0;

import "@uniswap/v2-core/contracts/interfaces/IUniswapV2Pair.sol";

contract PortalPrecompileInterface {
    enum PortalPrecompileInterfaceEnum {
        GetLatestFinalizedHeader,
        GetLatestFinalizedHeight,
        GetLatestUpdatedHeight,
        GetCurrentEpoch,
        ReadFastConfirmationOffset,
        ReadRationalConfirmationOffset,
        ReadEpochOffset,
        VerifyEventInclusion,
        VerifyStateInclusion,
        VerifyTxInclusion
    }

    struct ChainId {
        uint256 value;
    }

    struct Bytes {
        bytes value;
    }

    struct VerifyTxInclusionData {
        bytes4 id;
        Bytes data;
    }

    struct ReadPortalOption {
        PortalPrecompileInterfaceEnum action;
        bytes4 id;
    }

    function encodeReadPrecompileSelect(ReadPortalOption calldata t) public pure returns (bytes memory) {
        return abi.encodePacked(uint8(t.action), t.id);
    }

    struct VerifyInclusionPortalOption {
        PortalPrecompileInterfaceEnum action;
        bytes4 id;
        bytes data;
    }

    function encodeVerifyPrecompileSelect(VerifyInclusionPortalOption memory t, address targetContractAddress) public pure returns (bytes memory) {
        return abi.encodePacked(uint8(t.action), t.id, targetContractAddress, t.data);
    }
}

contract Snorkle is PortalPrecompileInterface {
    // event interface
    string private assetPriceInterface = "AssetPrice(address pairAddress, uint112 reserveAsset0, uint112 reserveAsset0, uint32 timestamp)";

    struct AssetPrice {
        address pairAddress; // pool contract address
        uint112 reserveAsset0;
        uint112 reserveAsset1;
        uint32 timestamp;
    }

    address private constant PORTAL = address(0x000000000000000000000000000000000000000A); // address of precompile

    mapping(address => AssetPrice) public currentAssetPrice; // mapping of current prices

    address targetContractAddress; // address of the pricing contract on target
    bytes4 target; // id of target chain

    event NewAssetPrice(AssetPrice);

    constructor(address _targetContractAddress, bytes4 _target) {
        targetContractAddress = _targetContractAddress;
        target = _target;
    }

    function addAssetPrice(bytes calldata proof) public {
        (bool success, bytes memory returnData) = PORTAL.staticcall(
            PortalPrecompileInterface.encodeVerifyPrecompileSelect(
                VerifyInclusionPortalOption(
                    PortalPrecompileInterfaceEnum.VerifyEventInclusion,
                    target,
                    proof
                ),
                targetContractAddress
            )
        );

        if(success) {
            AssetPrice memory lastestPrice = abi.decode(returnData, (AssetPrice));
            currentAssetPrice[lastestPrice.pairAddress] = lastestPrice;
            emit NewAssetPrice(lastestPrice);
        } else {
            revert();
        }
    }

}

contract SnorkleCollector {
    struct AssetPrice {
        address pairAddress; // pool contract address
        uint112 reserveAsset0;
        uint112 reserveAsset1;
        uint32 timestamp;
    }

    // the price update event
    event PriceUpdate(AssetPrice);


    function fetchNewPrice(address pairAddress) public {
        // get reserves and token order from pair contract
        (uint112 reserve0, uint112 reserve1, uint32 timestamp) = IUniswapV2Pair(pairAddress).getReserves();

        // emit price update event
        emit PriceUpdate(AssetPrice(pairAddress, reserve0, reserve1, timestamp));
    }
}

pragma solidity ^0.8.0;

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
        bytes memory encodedData = abi.encodePacked(uint8(t.action), t.id);
        return encodedData;
    }

    struct VerifyInclusionPortalOption {
        PortalPrecompileInterfaceEnum action;
        bytes4 id;
        Bytes data;
    }
    function encodeVerifyPrecompileSelect(VerifyInclusionPortalOption calldata t) public pure returns (bytes memory) {
        bytes memory encodedData = abi.encodePacked(uint8(t.action), t.id, t.data.value);
        return encodedData;
    }
}

pragma solidity ^0.8.0;

contract PortalPrecompileInterface {
    enum PortalPrecompileInterfaceEnum {
        GetLatestFinalizedHeader,
        GetFinalizedHeight,
        GetRationalHeight,
        GetFastHeight,
        VerifyEventInclusion,
        VerifyStateInclusion,
        VerifyTxInclusion
    }

    enum SpeedMode {
        Fast,
        Rational,
        Finalized
    }

    struct ChainId {
        uint256 value;
    }

    struct Bytes {
        bytes value;
    }

    struct ReadPortalOption {
        PortalPrecompileInterfaceEnum action;
        bytes4 id;
    }

    function encodeReadPrecompileSelect(ReadPortalOption memory t) public pure returns (bytes memory) {
        bytes memory encodedData = abi.encodePacked(uint8(t.action), t.id);
        return encodedData;
    }

    function encodeGetLatestFinalizedHeader(bytes4 gateway_id) public pure returns (bytes memory) {
        ReadPortalOption memory t = ReadPortalOption(PortalPrecompileInterfaceEnum.GetLatestFinalizedHeader, gateway_id);
        return encodeReadPrecompileSelect(t);
    }

    function encodeGetFinalizedHeight(bytes4 gateway_id) public pure returns (bytes memory) {
        ReadPortalOption memory t = ReadPortalOption(PortalPrecompileInterfaceEnum.GetFinalizedHeight, gateway_id);
        return encodeReadPrecompileSelect(t);
    }

    function encodeGetRationalHeight(bytes4 gateway_id) public pure returns (bytes memory) {
        ReadPortalOption memory t = ReadPortalOption(PortalPrecompileInterfaceEnum.GetRationalHeight, gateway_id);
        return encodeReadPrecompileSelect(t);
    }

    function encodeGetFastHeight(bytes4 gateway_id) public pure returns (bytes memory) {
        ReadPortalOption memory t = ReadPortalOption(PortalPrecompileInterfaceEnum.GetFastHeight, gateway_id);
        return encodeReadPrecompileSelect(t);
    }

    struct VerifyInclusionPortalOption {
        PortalPrecompileInterfaceEnum action;
        bytes4 id;
        SpeedMode speedMode;
        bytes32 executionSource;
        Bytes data;
    }

    function encodeVerifyPrecompileSelect(VerifyInclusionPortalOption memory t) public pure returns (bytes memory) {
        bytes memory encodedData = abi.encodePacked(uint8(t.action), t.id, uint8(t.speedMode), t.executionSource, t.data.value);
        return encodedData;
    }

    function encodeVerifyEventInclusion(bytes4 gateway_id, SpeedMode speedMode, Bytes memory event_data) public pure returns (bytes memory) {
        return encodeVerifyEventInclusionWithSource(gateway_id, speedMode, event_data, address(0));
    }

    function encodeVerifyEventInclusionWithSource(bytes4 gateway_id, SpeedMode speedMode, Bytes memory event_data, address executionSource) public pure returns (bytes memory) {
        VerifyInclusionPortalOption memory t = VerifyInclusionPortalOption(PortalPrecompileInterfaceEnum.VerifyEventInclusion, gateway_id, speedMode, bytes32(uint256(uint160(executionSource)) << 96), event_data);
        return encodeVerifyPrecompileSelect(t);
    }

    function encodeVerifyStateInclusionWithSource(bytes4 gateway_id, SpeedMode speedMode, Bytes memory event_data, address executionSource) public pure returns (bytes memory) {
        VerifyInclusionPortalOption memory t = VerifyInclusionPortalOption(PortalPrecompileInterfaceEnum.VerifyStateInclusion, gateway_id, speedMode, bytes32(uint256(uint160(executionSource)) << 96), event_data);
        return encodeVerifyPrecompileSelect(t);
    }

    function encodeVerifyStateInclusion(bytes4 gateway_id, SpeedMode speedMode, Bytes memory event_data) public pure returns (bytes memory) {
        return encodeVerifyStateInclusionWithSource(gateway_id, speedMode, event_data, address(0));
    }

    function encodeVerifyTxInclusionWithSource(bytes4 gateway_id, SpeedMode speedMode, Bytes memory event_data, address executionSource) public pure returns (bytes memory) {
        VerifyInclusionPortalOption memory t = VerifyInclusionPortalOption(PortalPrecompileInterfaceEnum.VerifyTxInclusion, gateway_id, speedMode, bytes32(uint256(uint160(executionSource)) << 96), event_data);
        return encodeVerifyPrecompileSelect(t);
    }

    function encodeVerifyTxInclusion(bytes4 gateway_id, SpeedMode speedMode, Bytes memory event_data) public pure returns (bytes memory) {
        return encodeVerifyTxInclusionWithSource(gateway_id, speedMode, event_data, address(0));
    }
}

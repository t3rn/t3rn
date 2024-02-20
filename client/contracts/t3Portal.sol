// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.20;

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

    function encodeReadPrecompileMemorySelect(ReadPortalOption memory t) public pure returns (bytes memory) {
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

contract t3Portal is PortalPrecompileInterface {

    address private constant PORTAL = address(0x0808080808080808080808080808080808080808); // address of portal precompile

    function getLatestFinalizedHeader(bytes4 chainId) public view returns (bytes memory) {
        (bool success, bytes memory returnData) = PORTAL.staticcall(
            encodeReadPrecompileMemorySelect(ReadPortalOption(PortalPrecompileInterfaceEnum.GetLatestFinalizedHeader, chainId))
        );
        if (success) {
            return returnData;
        } else {
            return "";
        }
    }

    function getLatestFinalizedHeight(bytes4 chainId) public view returns (bytes memory) {
        (bool success, bytes memory returnData) = PORTAL.staticcall(
            encodeReadPrecompileMemorySelect(ReadPortalOption(PortalPrecompileInterfaceEnum.GetLatestFinalizedHeight, chainId))
        );
        if (success) {
            return returnData;
        } else {
            return "";
        }
    }

    function getLatestUpdatedHeight(bytes4 chainId) public view returns (bytes memory) {
        (bool success, bytes memory returnData) = PORTAL.staticcall(
            encodeReadPrecompileMemorySelect(ReadPortalOption(PortalPrecompileInterfaceEnum.GetLatestUpdatedHeight, chainId))
        );
        if (success) {
            return returnData;
        } else {
            return "";
        }
    }

    function getCurrentEpoch(bytes4 chainId) public view returns (bytes memory) {
        (bool success, bytes memory returnData) = PORTAL.staticcall(
            encodeReadPrecompileMemorySelect(ReadPortalOption(PortalPrecompileInterfaceEnum.GetCurrentEpoch, chainId))
        );
        if (success) {
            return returnData;
        } else {
            return "";
        }
    }

    function readFastConfirmationOffset(bytes4 chainId) public view returns (bytes memory) {
        (bool success, bytes memory returnData) = PORTAL.staticcall(
            encodeReadPrecompileMemorySelect(ReadPortalOption(PortalPrecompileInterfaceEnum.ReadFastConfirmationOffset, chainId))
        );
        if (success) {
            return returnData;
        } else {
            return "";
        }
    }

    function readRationalConfirmationOffset(bytes4 chainId) public view returns (bytes memory) {
        (bool success, bytes memory returnData) = PORTAL.staticcall(
            encodeReadPrecompileMemorySelect(ReadPortalOption(PortalPrecompileInterfaceEnum.ReadRationalConfirmationOffset, chainId))
        );
        if (success) {
            return returnData;
        } else {
            return "";
        }
    }

    function readEpochOffset(bytes4 chainId) public view returns (bytes memory) {
        (bool success, bytes memory returnData) = PORTAL.staticcall(
            encodeReadPrecompileMemorySelect(ReadPortalOption(PortalPrecompileInterfaceEnum.ReadEpochOffset, chainId))
        );
        if (success) {
            return returnData;
        } else {
            return "";
        }
    }

    function verifyEventInclusion(bytes4 chainId, bytes memory proof, address maybeCheckProofSource) public view returns (bytes memory) {
        (bool success, bytes memory returnData) = PORTAL.staticcall(
            encodeVerifyPrecompileSelect(VerifyInclusionPortalOption(PortalPrecompileInterfaceEnum.VerifyEventInclusion, chainId, proof), maybeCheckProofSource)
        );
        if (success) {
            return returnData;
        } else {
            return "";
        }
    }

    function verifyStateInclusion(bytes4 chainId, bytes memory proof, address maybeCheckProofSource) public view returns (bytes memory) {
        (bool success, bytes memory returnData) = PORTAL.staticcall(
            encodeVerifyPrecompileSelect(VerifyInclusionPortalOption(PortalPrecompileInterfaceEnum.VerifyStateInclusion, chainId, proof), maybeCheckProofSource)
        );
        if (success) {
            return returnData;
        } else {
            return "";
        }
    }

    function verifyTxInclusion(bytes4 chainId, bytes memory proof, address maybeCheckProofSource) public view returns (bytes memory) {
        (bool success, bytes memory returnData) = PORTAL.staticcall(
            encodeVerifyPrecompileSelect(VerifyInclusionPortalOption(PortalPrecompileInterfaceEnum.VerifyTxInclusion, chainId, proof), maybeCheckProofSource)
        );
        if (success) {
            return returnData;
        } else {
            return "";
        }
    }
}
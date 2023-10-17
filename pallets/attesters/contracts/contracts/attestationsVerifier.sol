// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

contract AttestationsVerifier {

    event SignerEmitted(address indexed signer);
    event BatchApplied(bytes32 indexed batchHash, address indexed executor);
    event SignerNotInCommittee(address indexed signer);

    struct Batch {
        address[] nextCommittee;
        address[] bannedCommittee;
        bytes32[] committedSfx;
        bytes32[] revertedSfx;
        uint32 index;
    }

    mapping(bytes32 => bool) public committedSfxMap;
    mapping(bytes32 => bool) public committedGMPMap;
    mapping(bytes32 => bool) public revertedSfxMap;
    mapping(address => uint256) public attestersIndices;
    address public owner;
    address public escrowGMP;
    uint256 public committeeSize;
    uint256 public currentCommitteeTransitionCount;
    uint256 public currentBatchIndex;
    uint256 public totalAttesters; // added a counter to track total attestors.

    function batchEncodePacked(Batch memory batch) public pure returns (bytes memory) {
        return abi.encodePacked(
            batch.nextCommittee,
            batch.bannedCommittee,
            batch.committedSfx,
            batch.revertedSfx,
            batch.index
        );
    }

    function messageHash(Batch memory batch) public pure returns (bytes32) {
        return keccak256(batchEncodePacked(batch));
    }

    constructor(address[] memory initialCommittee, uint256 startingIndex) {
        currentCommitteeTransitionCount = 1;
        for(uint i = 0; i < initialCommittee.length; i++) {
            attestersIndices[initialCommittee[i]] = currentCommitteeTransitionCount;
        }
        totalAttesters = initialCommittee.length;
        currentBatchIndex = startingIndex;
        owner = msg.sender;
        committeeSize = initialCommittee.length;
    }

    function updateCommitteeSize(uint256 newCommitteeSize) public {
        require(msg.sender == owner, "Only owner can update committee size");
        committeeSize = newCommitteeSize;
    }

    function receiveAttestationBatch(
        address[] memory nextCommittee,
        address[] memory bannedCommittee,
        bytes32[] memory committedSfx,
        bytes32[] memory revertedSfx,
        uint32 index,
        bytes32 expectedBatchHash,
        bytes[] memory signatures
    ) public {
        Batch memory batch = Batch(nextCommittee, bannedCommittee, committedSfx, revertedSfx, index);
        bytes32 batchMessageHash = keccak256(batchEncodePacked(batch));
        require(batchMessageHash == expectedBatchHash, "Batch hash mismatch");

        require(batch.index == currentBatchIndex + 1, "Batch index mismatch");

        require(verifySignedByActiveCommittee(expectedBatchHash, signatures), "Signatures verification failed");

        if (batch.nextCommittee.length > 0) {
            currentCommitteeTransitionCount += 1;
            // Add new attesters to the attestersIndices mapping
            for(uint i = 0; i < batch.nextCommittee.length; i++) {
                uint256 attesterIndex = attestersIndices[batch.nextCommittee[i]];
                if (attesterIndex == 0) {
                    totalAttesters += 1;
                }
                if (attesterIndex != type(uint256).max) {
                    attestersIndices[batch.nextCommittee[i]] = currentCommitteeTransitionCount;
                }
            }
        }

        // Preserve the list of banned committee indices
        for(uint i = 0; i < batch.bannedCommittee.length; i++) {
            if (attestersIndices[batch.bannedCommittee[i]] != type(uint256).max) {
                attestersIndices[batch.bannedCommittee[i]] = type(uint256).max;
            }
        }

        for (uint i = 0; i < batch.committedSfx.length; i++) {
            committedSfxMap[batch.committedSfx[i]] = true;
        }

        for (uint i = 0; i < batch.revertedSfx.length; i++) {
            revertedSfxMap[batch.revertedSfx[i]] = true;
        }

        currentBatchIndex = batch.index;

        emit BatchApplied(expectedBatchHash, msg.sender);
    }

    function verifySignedByActiveCommittee(
        bytes32 messageHash,
        bytes[] memory signatures
    ) public returns (bool) {
        uint256 validSignatures = 0;
        uint256 quorum = committeeSize * 2 / 3;
        for (uint i = 0; i < signatures.length; i++) {
            bytes32 r;
            bytes32 s;
            uint8 v;

            bytes memory signature = signatures[i];

            if (signature.length != 65) {
                continue;
            }

            assembly {
                r := mload(add(signature, 32))
                s := mload(add(signature, 64))
                v := byte(0, mload(add(signature, 96)))
            }

            if (v < 27) {
                v += 27;
            }

            if (v != 27 && v != 28) {
                continue;
            } else {
                bytes32 prefixedHash = keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", messageHash));
                address signer = ecrecover(prefixedHash, v, r, s);
                uint256 attesterIndex = attestersIndices[signer];

                if (attesterIndex == currentCommitteeTransitionCount) {
                    validSignatures += 1;
                }
            }

            if (validSignatures >= quorum) {
                return true;
            }
        }

        return validSignatures >= quorum;
    }

    function arrayContains(uint256[] memory array, uint256 value) private pure returns (bool) {
        for(uint256 i = 0; i < array.length; i++) {
            if(array[i] == value) {
                return true;
            }
        }
        return false;
    }

    function recoverSigner(bytes32 messageHash, bytes memory signature) public pure returns (address) {
        bytes32 r;
        bytes32 s;
        uint8 v;

        if (signature.length != 65) {
            return address(0);
        }

        assembly {
            r := mload(add(signature, 32))
            s := mload(add(signature, 64))
            v := byte(0, mload(add(signature, 96)))
        }

        if (v < 27) {
            v += 27;
        }

        if (v != 27 && v != 28) {
            return address(0);
        } else {
            bytes32 prefixedHash = keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", messageHash));
            return ecrecover(prefixedHash, v, r, s);
        }
    }
}

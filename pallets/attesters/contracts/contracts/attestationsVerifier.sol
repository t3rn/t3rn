// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

contract AttestationsVerifier {
    event SignerEmitted(address indexed signer);
    event BatchApplied(bytes32 indexed batchHash, uint256 numSignatures);
    event SignerNotInCommittee(address indexed signer);

    mapping(address => bool) public current_committee;

    struct Batch {
        address[] nextCommittee;
        address[] bannedCommittee;
        bytes32[] committedSfx;
        bytes32[] revertedSfx;
        uint32 index;
    }

    mapping(bytes32 => bool) public committedSfxMap;
    mapping(bytes32 => bool) public revertedSfxMap;
    uint256 public committeeSize;
    uint256 public currentBatchIndex;

    function batchEncodePacked(Batch memory batch) public pure returns (bytes memory) {
        return abi.encodePacked(
            batch.nextCommittee,
            batch.bannedCommittee,
            batch.committedSfx,
            batch.revertedSfx,
            batch.index
        );
    }

    constructor(address[] memory initialCommittee) {
        for (uint i = 0; i < initialCommittee.length; i++) {
            current_committee[initialCommittee[i]] = true;
        }
        committeeSize = initialCommittee.length;
        currentBatchIndex = 0;
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
        // TODO: 
        // bytes32 batchMessageHash = keccak256(batchEncodePacked(batch));
        // require(batchMessageHash == expectedBatchHash, "Batch hash mismatch");

        require(verifySignedByActiveCommittee(expectedBatchHash, signatures), "Signatures verification failed");

        require(batch.index == currentBatchIndex, "Batch index mismatch");

        uint256 _committeeSize = committeeSize;
        for (uint i = 0; i < batch.nextCommittee.length; i++) {
            if (!current_committee[batch.nextCommittee[i]]) {
                current_committee[batch.nextCommittee[i]] = true;
                _committeeSize += 1;
            }
        }

        for (uint i = 0; i < batch.bannedCommittee.length; i++) {
            if (current_committee[batch.bannedCommittee[i]]) {
                delete current_committee[batch.bannedCommittee[i]];
                _committeeSize -= 1;
            }
        }

        committeeSize = _committeeSize;

        for (uint i = 0; i < batch.committedSfx.length; i++) {
            committedSfxMap[batch.committedSfx[i]] = true;
        }

        for (uint i = 0; i < batch.revertedSfx.length; i++) {
            revertedSfxMap[batch.revertedSfx[i]] = true;
        }

        currentBatchIndex = batch.index + 1;

        emit BatchApplied(expectedBatchHash, signatures.length);
    }

    function verifySignedByActiveCommittee(
        bytes32 messageHash,
        bytes[] memory signatures
    ) public returns (bool) {
        uint256 validSignatures = 0;
        uint256 quorum = committeeSize * 2 / 3;
        for (uint i = 0; i < signatures.length; i++) {
            address signer = recoverSigner(messageHash, signatures[i]);
            if (signer != address(0) && current_committee[signer]) {
                validSignatures += 1;
                if (validSignatures >= quorum) {
                    return true;
                }
            }
        }
        return validSignatures > committeeSize * 2 / 3;
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

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

contract AttestationsVerifier {
    event SignerEmitted(address indexed signer);
    event BatchApplied(bytes32 indexed batchHash, uint256 numSignatures);
    event SignerNotInCommittee(address indexed signer);

    mapping(address => bool) public current_committee;

    struct Batch {
        address[] newCommittee;
        address[] bannedCommittee;
        bytes32[] confirmedSFXs;
        bytes32[] revertedSFXs;
        uint32 index;
    }

    mapping(bytes32 => bool) public confirmedSFXsMap;
    mapping(bytes32 => bool) public revertedSFXsMap;
    uint256 public committeeSize;
    uint256 public currentBatchIndex;

    function batchEncodePacked(Batch memory batch) public pure returns (bytes memory) {
        return abi.encodePacked(
            batch.newCommittee,
            batch.bannedCommittee,
            batch.confirmedSFXs,
            batch.revertedSFXs,
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
        address[] memory newCommittee,
        address[] memory bannedCommittee,
        bytes32[] memory confirmedSFXs,
        bytes32[] memory revertedSFXs,
        uint32 index,
        bytes32 expectedBatchHash,
        bytes[] memory signatures
    ) public {
        Batch memory batch = Batch(newCommittee, bannedCommittee, confirmedSFXs, revertedSFXs, index);
        bytes32 batchMessageHash = keccak256(batchEncodePacked(batch));
        require(batchMessageHash == expectedBatchHash, "Batch hash mismatch");

        require(verifySignedByActiveCommittee(batchMessageHash, signatures), "Signatures verification failed");

        require(batch.index == currentBatchIndex + 1, "Batch index mismatch");

        uint256 _committeeSize = committeeSize;
        for (uint i = 0; i < batch.newCommittee.length; i++) {
            if (!current_committee[batch.newCommittee[i]]) {
                current_committee[batch.newCommittee[i]] = true;
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

        for (uint i = 0; i < batch.confirmedSFXs.length; i++) {
            confirmedSFXsMap[batch.confirmedSFXs[i]] = true;
        }

        for (uint i = 0; i < batch.revertedSFXs.length; i++) {
            revertedSFXsMap[batch.revertedSFXs[i]] = true;
        }

        currentBatchIndex = batch.index;

        emit BatchApplied(batchMessageHash, signatures.length);
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

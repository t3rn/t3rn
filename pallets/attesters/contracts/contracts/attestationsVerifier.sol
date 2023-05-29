// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract AttestationsVerifier {
    event SignerEmitted(address indexed signer, uint32 indexed memberIndex);
    event BatchApplied(bytes32 indexed batchHash, uint256 numSignatures);
    event CommitteeTransitioned(address[] newCommittee);
    event InvalidSignature(address indexed invalidSigner, bytes signature);
    event ReceivedSignatures(uint256 numSignatures);
    event RecoveredSigner(address indexed signer);
    event SignerNotInCommittee(address indexed signer);

    struct CommitteeMember {
        uint32 index;
        bool isMember;
    }

    mapping(address => CommitteeMember) public current_committee;

    struct Attester {
        address attesterAddress;
        uint32 index;
    }

    struct Batch {
        Attester[] newCommittee;
        address[] bannedCommittee;
        bytes32[] confirmedSFXs;
    }

    mapping(bytes32 => bool) public confirmedSFXs;
    uint256 public committeeSize;

    constructor(Attester[] memory initialCommittee) {
        for (uint i = 0; i < initialCommittee.length; i++) {
            current_committee[initialCommittee[i].attesterAddress] = CommitteeMember(initialCommittee[i].index, true);
        }
        committeeSize = initialCommittee.length;
    }

    function receiveAttestationBatch(
        bytes memory batchMessage,
        bytes32 expectedBatchHash,
        bytes[] memory signatures
    ) public {
        bytes32 batchMessageHash = keccak256(abi.encodePacked(batchMessage));
        require(batchMessageHash == expectedBatchHash, "Batch hash mismatch");
        Batch memory batch = abi.decode(batchMessage, (Batch));

        require(verifySignedByActiveCommittee(batchMessageHash, signatures), "Signatures verification failed");

        for (uint i = 0; i < batch.newCommittee.length; i++) {
            if (!current_committee[batch.newCommittee[i].attesterAddress].isMember) {
                current_committee[batch.newCommittee[i].attesterAddress] = CommitteeMember(batch.newCommittee[i].index, true);
                committeeSize += 1;
            }
        }

        for (uint i = 0; i < batch.bannedCommittee.length; i++) {
            if (current_committee[batch.bannedCommittee[i]].isMember) {
                delete current_committee[batch.bannedCommittee[i]];
                committeeSize -= 1;
            }
        }

        for (uint i = 0; i < batch.confirmedSFXs.length; i++) {
            confirmedSFXs[batch.confirmedSFXs[i]] = true;
        }

        emit BatchApplied(batchMessageHash, signatures.length);
    }

    function verifySignedByActiveCommittee(
        bytes32 messageHash,
        bytes[] memory signatures
    ) public returns (bool) {
        emit ReceivedSignatures(signatures.length);
        uint256 validSignatures = 0;
        for (uint i = 0; i < signatures.length; i++) {
            address signer = recoverSigner(messageHash, signatures[i]);
            emit RecoveredSigner(signer);
            if (signer == address(0)) {
                emit InvalidSignature(signer, signatures[i]);
            }
            if (current_committee[signer].isMember) {
                emit SignerEmitted(signer, current_committee[signer].index);
                validSignatures += 1;
            } else {
                emit SignerNotInCommittee(signer);
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

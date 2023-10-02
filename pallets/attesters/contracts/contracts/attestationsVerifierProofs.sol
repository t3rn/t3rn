// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;
import "@openzeppelin/contracts/utils/cryptography/MerkleProof.sol";

interface IEscrowGMP {
    function commitRemoteExecutorPayload(bytes32 sfxId, address executor) external returns (bool);
    function revertRemoteExecutorPayload(bytes32 sfxId) external returns (bool);
    function mintOrBurnToVault(bool mint, bytes32 sfxId, uint256 amount, address mintContract) external returns (bool);
}

contract AttestationsVerifierProofs {
    using MerkleProof for bytes[];

    event SignerEmitted(address indexed signer);
    event TestEvent(bool, bool, bytes32[] leaves, address[] addressesRecovered, bytes32);
    event BatchApplied(bytes32 indexed batchHash, address indexed executor);
    event SignerNotInCommittee(address indexed signer);

    struct Batch {
        bool is_halted;
        bytes32 currentCommitteeHash;
        bytes32 nextCommitteeHash;
        address[] bannedCommittee;
        bytes32[][2] committedSfx;
        bytes32[] revertedSfx;
        uint256[] priceUpdates;
        uint32 index;
        bytes encodedGMPPayload;
    }

    struct PriceEntry {
        uint256 priceInETH; // This will represent how much 1 unit of the asset is worth in ETH.
        uint256 lastUpdated; // Timestamp of the last update.
    }

    function updatePrice(string memory assetName, uint256 newPrice) internal {
        prices[assetName].priceInETH = newPrice;
        prices[assetName].lastUpdated = block.timestamp;
    }

    function batchUpdatePrices(string[] memory assetNames, uint256[] memory newPrices) internal {
        require(assetNames.length == newPrices.length, "Mismatched arrays");

        for(uint256 i = 0; i < assetNames.length; i++) {
            updatePrice(assetNames[i], newPrices[i]);
        }
    }

    function getPrice(string memory assetName) external view returns (uint256) {
        return prices[assetName].priceInETH;
    }

    function getPriceUpdateTime(string memory assetName) external view returns (uint256) {
        return prices[assetName].lastUpdated;
    }


    mapping(address => uint256) public attestersIndices;
    mapping(string => PriceEntry) public prices;

    address public owner;
    uint256 public committeeSize;
    uint256 public quorum;
    uint256 public currentCommitteeTransitionCount;
    uint256 public currentBatchIndex;
    uint256 public totalAttesters; // added a counter to track total attestors.
    bytes32 public currentCommitteeHash;
    bytes32 public nextCommitteeHash;
    IEscrowGMP private escrowGMP;

    function batchEncodePacked(Batch memory batch) public pure returns (bytes memory) {
        return abi.encode(
            batch.is_halted,
            batch.currentCommitteeHash,
            batch.nextCommitteeHash,
            batch.bannedCommittee,
            batch.committedSfx,
            batch.revertedSfx,
            batch.encodedGMPPayload,
            batch.index
        );
    }

    function messageHash(Batch memory batch) public pure returns (bytes32) {
        return keccak256(batchEncodePacked(batch));
    }

    constructor(address[] memory initialCommittee, address[] memory nextCommittee, uint256 startingIndex, address _escrowGMP) {
        currentCommitteeTransitionCount = 1;
        for(uint i = 0; i < initialCommittee.length; i++) {
            attestersIndices[initialCommittee[i]] = currentCommitteeTransitionCount;
        }
        totalAttesters = initialCommittee.length;
        currentBatchIndex = startingIndex;
        owner = msg.sender;
        committeeSize = initialCommittee.length;
        quorum = committeeSize * 2 / 3;
        currentCommitteeHash = implyCommitteeRoot(initialCommittee);
        nextCommitteeHash = implyCommitteeRoot(nextCommittee);
        escrowGMP = IEscrowGMP(_escrowGMP);
    }

    function implyCommitteeRoot(address[] memory committee) public pure returns (bytes32) {
        bytes32[] memory leaves = new bytes32[](committee.length);
        for (uint256 i = 0; i < committee.length; i++) {
            leaves[i] = keccak256(bytes.concat(keccak256(abi.encode(committee[i]))));
        }
        bytes32[] memory multiProofProof = new bytes32[](0);
        bool[] memory multiProofMembershipFlags = new bool[](committee.length - 1);
        for (uint256 i = 0; i < committee.length - 1; i++) {
            multiProofMembershipFlags[i] = true;
        }

        uint256 leavesLen = leaves.length;
        uint256 proofLen = multiProofProof.length;
        uint256 totalHashes = multiProofMembershipFlags.length;
        require(leavesLen + proofLen == totalHashes + 1, "Invalid multiProofProof proof length");

        return MerkleProof.processMultiProof(multiProofProof, multiProofMembershipFlags, leaves);
    }

    function updateCommitteeSize(uint256 newCommitteeSize) public {
        require(msg.sender == owner, "Only owner can update committee size");
        committeeSize = newCommitteeSize;
        quorum = committeeSize * 2 / 3;
    }

    function receiveAttestationBatch(
        uint32 index,
        bytes calldata batchPayload,
        bytes[] calldata signatures,
        bytes32[] calldata multiProofProof,
        bool[] calldata multiProofMembershipFlags,
        address[] calldata maybeNextCommittee
    ) public {
        Batch memory batch = abi.decode(batchPayload, (Batch));

        bytes32 batchMessageHash = keccak256(batchEncodePacked(batch));
        require(index == currentBatchIndex + 1, "Batch index mismatch");

        bytes32[] memory attestersAsLeaves = recoverCurrentSigners(
            batchMessageHash,
            signatures,
            batch.bannedCommittee
        );

        // Check if maybeNextCommittee contains new members commitment
        // If so, use the currently store next committee hash to verify signatures
        // Otherwise, use the currently stored current committee hash to verify signatures
        if (maybeNextCommittee.length > 0) {
            bytes32 impliedNextCommitteeHash = implyCommitteeRoot(maybeNextCommittee);
            require(impliedNextCommitteeHash == nextCommitteeHash, "Next committee hash mismatch");
            require(MerkleProof.multiProofVerifyCalldata(multiProofProof, multiProofMembershipFlags, impliedNextCommitteeHash, attestersAsLeaves), "Multi-proof of attestations commitments verification failed");
            currentCommitteeHash = nextCommitteeHash;
            nextCommitteeHash = impliedNextCommitteeHash;
        } else {
            require(MerkleProof.multiProofVerifyCalldata(multiProofProof, multiProofMembershipFlags, currentCommitteeHash, attestersAsLeaves), "Multi-proof of attestations commitments verification failed");
        }

        decodeAndProcessPayload(batchPayload);

        // Check if batch includes price updates and apply them
        if (batch.priceUpdates.length > 0) {
            require(batch.priceUpdates.length == 2, "Invalid price updates length");
            updatePrice("xTRN", batch.priceUpdates[0]);
            updatePrice("xDOT", batch.priceUpdates[1]);
        }

        currentBatchIndex = batch.index;

        emit BatchApplied(batchMessageHash, msg.sender);
    }

    function recoverCurrentSigners(
        bytes32 expectedBatchHash,
        bytes[] calldata signatures,
        address[] memory bannedCommittee
    ) public returns (bytes32[] memory leaves) {
        uint32 correctSignatures = 0;
        bytes32[] memory leaves = new bytes32[](signatures.length);
        address[] memory recoveredAddresses = new address[](signatures.length);
        for (uint256 i = 0; i < signatures.length; i++) {
            address recoveredSigner = recoverSigner(expectedBatchHash, signatures[i]);
            require(recoveredSigner != address(0), "Bad signature");
            if (bannedCommittee.length > 0) {
                require(!addressArrayContains(bannedCommittee, recoveredSigner), "Signer is banned");
            }
            leaves[i] = keccak256(bytes.concat(keccak256(abi.encode(recoveredSigner))));
            recoveredAddresses[i] = recoveredSigner;
            correctSignatures += 1;
        }
        require(correctSignatures >= quorum, "Not enough correct signatures");
        return leaves;
    }

    function verifySignaturesTest(
        bytes32 expectedBatchHash,
        bytes[] calldata signatures,
        bytes32[] calldata multiProofProof,
        bool[] calldata multiProofMembershipFlags
    ) public {
        bytes32[] memory leaves = new bytes32[](signatures.length);
        address[] memory recoveredAddresses = new address[](signatures.length);
        for (uint256 i = 0; i < signatures.length; i++) {
            address recoveredSigner = recoverSigner(expectedBatchHash, signatures[i]);
            require(recoveredSigner != address(0), "Bad signature");
            leaves[i] = keccak256(bytes.concat(keccak256(abi.encode(recoveredSigner))));
            recoveredAddresses[i] = recoveredSigner;
        }
        require(MerkleProof.multiProofVerifyCalldata(multiProofProof, multiProofMembershipFlags, currentCommitteeHash, leaves), "Merkle proof verification failed");
    }

    enum OperationType { TransferCommit, TransferRevert, Mint, CallCommit, CallRevert }

    function decodeAndProcessPayload(bytes calldata payload) public {
        require(payload.length > 0, "Payload cannot be empty");

        uint256 offset = 0;
        while (offset < payload.length) {
            OperationType opType = OperationType(uint8(payload[offset]));
            bytes memory data;
            offset += 1;  // To move past the operation type byte

            if (opType == OperationType.TransferCommit) {
                require(payload.length >= offset + 52, "Payload too short for TransferCommit");
                data = bytes(payload[offset:offset+52]);  // 32 bytes for sfxId + 20 bytes for address
                bytes32 sfxId = bytes32(payload[offset:offset+32]);
                require(sfxId != bytes32(0), "Invalid sfxId");
                address destination = address(bytes20(payload[offset+32:offset+52]));
                require(destination != address(0), "Invalid destination");
                escrowGMP.commitRemoteExecutorPayload(sfxId, destination);
                offset += 52;
            } else if (opType == OperationType.TransferRevert) {
                require(payload.length >= offset + 32, "Payload too short for TransferRevert");
                data = bytes(payload[offset:offset+32]);  // 32 bytes for sfxId
                bytes32 sfxId = bytes32(payload[offset:offset+32]);
                require(sfxId != bytes32(0), "Invalid sfxId");
                escrowGMP.revertRemoteExecutorPayload(sfxId);
                offset += 32;
            } else if (opType == OperationType.Mint) {
                require(payload.length >= offset + 68, "Payload too short for Mint");
                data = bytes(payload[offset:offset+68]);  // 32 bytes for sfxId + 16 bytes for amount + 20 bytes for address
                bytes32 sfxId = bytes32(payload[offset:offset+32]);
                require(sfxId != bytes32(0), "Invalid sfxId");
                uint256 amount = uint256(uint128(bytes16(payload[offset+32:offset+48])));
                require(amount > 0, "Invalid amount");
                address destination = address(bytes20(payload[offset+52:offset+72]));
                require(destination != address(0), "Invalid destination");
                escrowGMP.mintOrBurnToVault(true, sfxId, amount, destination);
                offset += 32;
            } else if (opType == OperationType.CallCommit || opType == OperationType.CallRevert) {
                uint16 inputLength = uint16(bytes2(payload[offset:offset+2]));
                offset += 2;
                uint256 totalDataLength = 32 + 20 + inputLength;  // sfxId + destinationContract + input data
                require(payload.length >= offset + totalDataLength, "Payload too short for CallCommit/CallRevert");
                data = bytes(payload[offset:offset+totalDataLength]);
                if (opType == OperationType.CallCommit) {
                    processCallCommit(data);
                } else {
                    processCallRevert(data);
                }
                offset += totalDataLength;
            } else {
                revert("Invalid operation type");
            }
        }
    }

    struct TransferCommit {
        bytes32 sfxId;
        address executor;
    }

    struct TransferRevert {
        bytes32 sfxId;
    }

    struct CallCommit {
        bytes32 sfxId;
        address destinationContract;
        bytes input;
    }

    struct CallRevert {
        bytes32 sfxId;
        address destinationContract;
        bytes input;
    }

    function processCallCommit(bytes memory data) internal pure returns (CallCommit memory) {
        CallCommit memory commit;
        (commit.sfxId, commit.destinationContract, commit.input) = abi.decode(data, (bytes32, address, bytes));
        return commit;
    }

    function processCallRevert(bytes memory data) internal pure returns (CallRevert memory) {
        CallRevert memory revertData;
        (revertData.sfxId, revertData.destinationContract, revertData.input) = abi.decode(data, (bytes32, address, bytes));
        return revertData;
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


    function addressArrayContains(address[] memory array, address value) private pure returns (bool) {
        for(uint256 i = 0; i < array.length; i++) {
            if(array[i] == value) {
                return true;
            }
        }
        return false;
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

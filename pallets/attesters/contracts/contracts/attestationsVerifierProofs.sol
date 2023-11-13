// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;
import "@openzeppelin/contracts/utils/cryptography/MerkleProof.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "./escrowGMP.sol";

contract AttestationsVerifierProofs {
    using SafeERC20 for IERC20;

    using MerkleProof for bytes[];

    event SignerEmitted(address indexed signer);
    event TestEvent(bool, bool, bytes32[] leaves, address[] addressesRecovered, bytes32);
    event BatchApplied(bytes32 indexed batchHash, address indexed executor);
    event SignerNotInCommittee(address indexed signer);

    struct Batch {
        bool is_halted;
        bytes32 currentCommitteeHash;
        bytes32 nextCommitteeHash;
        address[] maybeNextCommittee;
        address[] bannedCommittee;
        bytes32 bannedStake;
        bytes32 newCommitteeStake;
        bytes32 priceUpdates;
        bytes encodedGMPPayload;
        uint32 index;
    }

    modifier onlyOwner() {
        require(msg.sender == owner, "Only owner can call this function");
        _;
    }

    // Return price in ETH for a given asset.
    function getQuote(string memory assetName, uint256 amountOfEth) external view returns (uint256) {
        if (keccak256(abi.encode(assetName)) == keccak256(abi.encode("TRN"))) {
            uint128[2] memory prices = abi.decode(abi.encode(currentTRNPriceAttestedInDOTAndETH), (uint128[2]));
            return amountOfEth * prices[1];
        } else if (keccak256(abi.encode(assetName)) == keccak256(abi.encode("DOT"))) {
            uint128[2] memory prices = abi.decode(abi.encode(currentTRNPriceAttestedInDOTAndETH), (uint128[2]));
            uint256 amountOfTrn = amountOfEth * (prices[1]);
            return amountOfTrn * prices[0];
        } else {
            revert("Invalid asset name");
        }
    }

    address public owner;
    address public xDOT;
    address public xTRN;
    uint256 public committeeSize;
    uint256 public quorum;
    uint256 public currentCommitteeTransitionCount;
    uint256 public currentBatchIndex;
    uint256 public totalAttesters; // added a counter to track total attestors.
    bytes32 public currentCommitteeHash;
    bytes32 public currentCommitteeStake; // staled (TRN | DOT) (uint64 | uint64) and 3-pool-liquid (TRN | DOT) (uint64 | uint64)
    bytes32 public nextCommitteeHash;
    bytes32 public currentTRNPriceAttestedInDOTAndETH;
    EscrowGMP private escrowGMP;

    function batchEncodePacked(Batch memory batch) public pure returns (bytes memory) {
        return abi.encode(
            batch.is_halted,
            batch.currentCommitteeHash,
            batch.nextCommitteeHash,
            batch.maybeNextCommittee,
            batch.bannedCommittee,
            batch.bannedStake,
            batch.newCommitteeStake,
            batch.priceUpdates,
            batch.encodedGMPPayload,
            batch.index
        );
    }

    function messageHash(Batch memory batch) public pure returns (bytes32) {
        return keccak256(batchEncodePacked(batch));
    }

    function singleAttestationHash(bytes calldata messageGMPPayload, bytes4 sourceGateway, uint32 sourceHeight) public pure returns (bytes32) {
        return keccak256(abi.encodePacked(keccak256(messageGMPPayload), sourceGateway, sourceHeight));
    }

    constructor(address[] memory initialCommittee, address[] memory nextCommittee, uint256 startingIndex, EscrowGMP _escrowGMP) {
        currentCommitteeTransitionCount = 1;
        totalAttesters = initialCommittee.length;
        currentBatchIndex = startingIndex;
        owner = msg.sender;
        committeeSize = initialCommittee.length;
        quorum = committeeSize * 2 / 3;
        if (initialCommittee.length > 0) {
            currentCommitteeHash = implyCommitteeRoot(initialCommittee);
        }
        if (nextCommittee.length > 0) {
            nextCommitteeHash = implyCommitteeRoot(nextCommittee);
        }
        escrowGMP = _escrowGMP;
    }

    function overrideCommitteeHash(bytes32 newCommitteeHash) public onlyOwner {
        currentCommitteeHash = newCommitteeHash;
    }

    function overrideNextCommitteeHash(bytes32 newCommitteeHash) public onlyOwner {
        nextCommitteeHash = newCommitteeHash;
    }

    function overrideCurrentBatchIndex(uint256 newBatchIndex) public onlyOwner {
        currentBatchIndex = newBatchIndex;
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

    function receiveSingleAttestation(
        bytes calldata messageGMPPayload, // keccak hash of the message constitutes 32bytes of message
        bytes4 sourceGateway, // 4 bytes of source gateway
        uint32 sourceHeight, // height of the block in which the message was emitted
        bytes[] calldata signatures,
        bytes32[] calldata multiProofProof,
        bool[] calldata multiProofMembershipFlags
    ) public {
        bytes32 messageHash = singleAttestationHash(messageGMPPayload, sourceGateway, sourceHeight);
        bytes32[] memory attestersAsLeaves = recoverCurrentSigners(
            messageHash,
            signatures,
            new address[](0)
        );
        require(attestersAsLeaves.length >= quorum, "Not enough correct signatures");

        require(MerkleProof.multiProofVerifyCalldata(multiProofProof, multiProofMembershipFlags, currentCommitteeHash, attestersAsLeaves), "Multi-proof of attestations commitments verification failed for single attestation");

        decodeAndProcessPayload(messageGMPPayload);
    }

    function receiveAttestationBatch(
        bytes calldata batchPayload,
        bytes calldata batchGMPPayload,
        bytes[] calldata signatures,
        bytes32[] calldata multiProofProof,
        bool[] calldata multiProofMembershipFlags
    ) public {
        Batch memory batch = abi.decode(batchPayload, (Batch));
        bytes32 batchMessageHash = keccak256(batchEncodePacked(batch));
        require(batch.index == currentBatchIndex + 1, "Batch index mismatch");

        bytes32[] memory attestersAsLeaves = recoverCurrentSigners(
            batchMessageHash,
            signatures,
            batch.bannedCommittee
        );

        require(attestersAsLeaves.length >= quorum, "Not enough correct signatures");

        // Check if maybeNextCommittee contains new members commitment
        // If so, use the currently store next committee hash to verify signatures
        // Otherwise, use the currently stored current committee hash to verify signatures
        if (batch.maybeNextCommittee.length > 0) {
            bytes32 impliedNextCommitteeHash = implyCommitteeRoot(batch.maybeNextCommittee);
            require(impliedNextCommitteeHash == nextCommitteeHash, "Next committee hash mismatch");
            require(MerkleProof.multiProofVerifyCalldata(multiProofProof, multiProofMembershipFlags, impliedNextCommitteeHash, attestersAsLeaves), "Multi-proof of attestations commitments verification failed");
            currentCommitteeHash = nextCommitteeHash;
            nextCommitteeHash = impliedNextCommitteeHash;
        } else {
            require(MerkleProof.multiProofVerifyCalldata(multiProofProof, multiProofMembershipFlags, currentCommitteeHash, attestersAsLeaves), "Multi-proof of attestations commitments verification failed");
        }

        decodeAndProcessPayload(batchGMPPayload);

        // Check if batch includes price updates and apply them
        if (batch.priceUpdates != bytes32(0)) {
            currentTRNPriceAttestedInDOTAndETH = batch.priceUpdates;
        }

        // Update the current committee stake if needed
        updateCurrentCommitteeStakeIfNeededBe(batch.bannedStake, batch.newCommitteeStake);

        currentBatchIndex = batch.index;

        emit BatchApplied(batchMessageHash, msg.sender);
    }

    function updateCurrentCommitteeStakeIfNeededBe(bytes32 bannedStake, bytes32 _newCommitteeStake) internal {
        if (_newCommitteeStake != bytes32(0)) {
            currentCommitteeStake = _newCommitteeStake;
        }
        if (bannedStake != bytes32(0)) {
            // decode current stake and subtract the banned stake from it.
            uint64[4] memory bannedStakeDecoded = abi.decode(abi.encode(bannedStake), (uint64[4]));
            uint64[4] memory currentStakeDecoded = abi.decode(abi.encode(currentCommitteeStake), (uint64[4]));
            uint64[4] memory newStakeDecoded;
            for (uint256 i = 0; i < 4; i++) {
                if (bannedStakeDecoded[i] > currentStakeDecoded[i]) {
                    newStakeDecoded[i] = 0;
                } else {
                    newStakeDecoded[i] = currentStakeDecoded[i] - bannedStakeDecoded[i];
                }
            }
            currentCommitteeStake = abi.decode(abi.encode(newStakeDecoded), (bytes32));
        }
    }

    function recoverCurrentSigners(
        bytes32 expectedBatchHash,
        bytes[] calldata signatures,
        address[] memory bannedCommittee
    ) public pure returns (bytes32[] memory leaves) {
        uint32 correctSignatures = 0;
        bytes32[] memory leaves = new bytes32[](signatures.length);
        for (uint256 i = 0; i < signatures.length; i++) {
            address recoveredSigner = recoverSigner(expectedBatchHash, signatures[i]);
            require(recoveredSigner != address(0), "Bad signature");
            if (bannedCommittee.length > 0) {
                require(!addressArrayContains(bannedCommittee, recoveredSigner), "Signer is banned");
            }
            leaves[i] = keccak256(bytes.concat(keccak256(abi.encode(recoveredSigner))));
            correctSignatures += 1;
        }
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

    function decodeAndProcessPayload(bytes calldata payload) public onlyOwner {
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
                escrowGMP.commitRemoteBeneficiaryPayload(sfxId, destination);
                offset += 52;
            } else if (opType == OperationType.TransferRevert) {
                require(payload.length >= offset + 32, "Payload too short for TransferRevert");
                data = bytes(payload[offset:offset+32]);  // 32 bytes for sfxId
                bytes32 sfxId = bytes32(payload[offset:offset+32]);
                require(sfxId != bytes32(0), "Invalid sfxId");
                escrowGMP.revertRemoteOrderPayload(sfxId);
                offset += 32;
            } else if (opType == OperationType.Mint) {
                require(payload.length >= offset + 88, "Payload too short for Mint");
                data = bytes(payload[offset:offset+88]);  // 32 bytes for sfxId + 16 bytes for amount + 20 bytes for address of mint contract + 20 bytes for address of destination
                bytes32 sfxId = bytes32(payload[offset:offset+32]);
                require(sfxId != bytes32(0), "Invalid sfxId");
                uint256 amount = uint256(uint128(bytes16(payload[offset+32:offset+48])));
                require(amount > 0, "Invalid amount");
                address destination = address(bytes20(payload[offset+48:offset+68]));
                require(destination != address(0), "Invalid destination");
                address beneficiary = address(bytes20(payload[offset+68:offset+88]));
                require(beneficiary != address(0), "Invalid destination");
                // Send xDOT from Vault address in ERC-20 to beneficiary, if amount does not exceed stake secured by current committee
                if (destination == xTRN) {
                    // Decode currentCommitteeStake for liquid and 3-pool assets collected in TRN
                    uint64[4] memory currentStakeDecoded = abi.decode(abi.encode(currentCommitteeStake), (uint64[4]));
                    uint256 totalTRNStake = currentStakeDecoded[0] + currentStakeDecoded[2];
                    if (amount < totalTRNStake) {
                        escrowGMP.withdrawFromVaultSkipGMPChecks(destination, amount, beneficiary);
                    }
                } else if (destination == xDOT) {
                    // Decode currentCommitteeStake for liquid and 3-pool assets collected in DOT
                    uint64[4] memory currentStakeDecoded = abi.decode(abi.encode(currentCommitteeStake), (uint64[4]));
                    uint256 totalDOTStake = currentStakeDecoded[1] + currentStakeDecoded[3];
                    if (amount < totalDOTStake) {
                        escrowGMP.withdrawFromVaultSkipGMPChecks(destination, amount, beneficiary);
                    }
                }
                offset += 88;
            } else if (opType == OperationType.CallCommit || opType == OperationType.CallRevert) {
                require(payload.length >= offset + 32, "Payload too short for CallCommit/CallRevert");
                data = bytes(payload[offset:offset+32]);  // 32 bytes for sfxId
                bytes32 sfxId = bytes32(payload[offset:offset+32]);
                require(sfxId != bytes32(0), "Invalid sfxId");
                if (opType == OperationType.CallCommit) {
                    escrowGMP.commitEscrowCall(sfxId);
                } else {
                    escrowGMP.revertEscrowCall(sfxId);
                }
                offset += 32;
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
    }

    struct CallRevert {
        bytes32 sfxId;
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

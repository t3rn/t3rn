pragma solidity ^0.8.5;
pragma experimental ABIEncoderv2; // allows us to pass structs
import "./SimplifiedMMRVerification.sol";
import "./HeaderRegistry.sol";

contract Escrow is SimplifiedMMRVerification, HeaderRegistry {

    // to save gas we hash the values
    mapping(bytes32 => bytes32) public activeEscrow;

    modifier noDuplicateXtx(xtxId) {
        require(!activeEscrowOps[xtxId].from, "Duplicate XTX ID");
        _;
    }

    event Commit(bytes32 xtxId);
    event Revert(bytes32 xtxId);

    struct CircuitEvent {
        // assumption here is, that the circut will only emit a commit event, if the amounts during execute are correct
        // need to specify what fields are required for finality proof
        bytes32 xtxId;
        bool shouldCommit;
    }

    function ethTransfer(address from, address to, bytes32 xtxId)
        external
        noDuplicateXtx(xtxId)
    {
        // we hash the inputs, this has two benefits
        // 1. reduces gas cost, as storage is more expensive then compute
        // 2. we can store all escrow types in same mapping. makes duplicate prevention checks a lot cheaper
        activeEscrowOps[xtxId] = _hashEthTransfer(from, to, msg.amount, msg.sender);
    }

    function tokenTransfer(address from, address to, address token, uint amount, bytes32 xtxId)
        external
        noDuplicateXtx(xtxId)
    {
        _collectToken(from, amount, token);
        activeEscrowOps[xtxId] = _hashTokenTransfer(from, to, token, amount, msg.sender);
    }

    function releaseEthTransfer(bytes32 xtxId, address from, address to, uint amount)
        external
    {
        // verify finality of circuit COMMIT/REVERT transaction here. See `_verifyFinality()`

        // ensure the correct inputs where passed
        require(_hashEthTransfer(from, to, amount, msg.sender) == activeEscrow[xtxId], "False inputs passed");

        if(evnt.shouldCommit) {
            // we are commiting
            _sendEth(to, amount);
        } else {   
            // reverting
            _sendEth(from, amount);
        }
         delete activeEscrow[xtxId]; // gas refund
    }

    function releaseTokenTransfer(bytes32 xtxId, address from, address to, address token, uint amount)
        external
    {
        // verify finality of circuit COMMIT/REVERT transaction here. See `_verifyFinality()`

        // ensure the correct inputs where passed
        require(_hashTokenTransfer(from, to, token, amount, msg.sender) == activeEscrow[xtxId], "False inputs passed");

        if(evnt.shouldCommit) {
            _sendToken(to, token, amount);
            emit Commit(xtxId);
        } else {   
            _sendToken(from, token, amount);
            emit Revert(xtxId);
        }

        delete activeEscrow[xtxId]; // gas refund
    }

    // this function is never called and simply a conceptual implementatio until we have the details figured out. 
    function _verifyFinality(bytes32 headerId, CircuitEvent memory evnt, SimplifiedMMRProof memory proof)
        private
    {
         // we assume the existance of a trustless header registry, containing verified t3rn block headers
        bytes32 root = HeaderRegstry.headers[headerId].root;

        // Hash the event/transaction to create leaf. Need more specs here.
        // I'm also assuming the circuit will ensure the correct escrow deposit was made (amount, assets, receiver, etc.).
        bytes32 leafHash = keccak256(abi.encodePacked(evnt.xtxId, evnt.shouldCommit));

        // Run imnclusion proof, prooving the finality and valifity of submitred event
        require(SimplifiedMMRVerification.verifyInclusionProof(root, leafHash, proof), "MMR verification failed.");
    }

    function _hashEthTransfer(address from, address to, uint amount, address executor)
        private
        returns (bytes32)
    {
        return keccak256(abi.encodePacked(from, to, amount, executor));
    }

    function _hashTokenTransfer(address from, address to, address asset, uint amount, address executor)
        private
        returns (bytes32)
    {
        return keccak256(abi.encodePacked(from, to, asset, amount, executor));
    }

    function _sendEth(address to, uint amount)
        private
    {
        to.owner.call{value: amount};
    }

    function _sendToken(address to, address token, uint amount)
        private
    {
        IERC20(token).transfer(to, amount);
    }

    function _collectToken(
        address from,
        uint256 amount,
        address token
    )
        private
    {
        // escrow contract needs to be approved before
        // This is how erc20 tokens are sent to smart contracts -> Approve contract as spender, then transferFrom to contract address.
        IERC20(token).transferFrom(from, address(this), amount);
    }
}
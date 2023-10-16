// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/utils/math/SafeMath.sol";
import "@openzeppelin/contracts/access/Ownable.sol";


interface AttestationVerifier {
    // This is a mockup of the functions you mentioned.
    // Your actual AttestationVerifier would have these implemented.
    function getAttestedEthCollateral() external view returns (uint256);

    function getPrice(string memory assetName) external view returns (uint256);

    function burnAttestedCollateral(string memory assetName, uint256 amount) external view returns (uint256);

    function getPriceUpdateTime(string memory assetName) external view returns (uint256);
}

contract xDOT is ERC20, Ownable {
    using SafeMath for uint256;

    event AttestedBridgeOrder(address indexed sender, string indexed assetName, uint256 indexed amount);
    AttestationVerifier public verifier;
    address public t3rnVault;

    constructor(address _verifier, address _t3rnVault) ERC20("xDOT Token", "xDOT") {
        verifier = AttestationVerifier(_verifier);
        t3rnVault = _t3rnVault;
    }

    function mintHere(uint256 ethAmount) external payable {
        require(ethAmount <= verifier.getAttestedEthCollateral(), "Insufficient collateral");

        require(msg.value == ethAmount, "Mismatched ETH amount");
        uint256 xdotPrice = verifier.getPrice("xDOT");
        require(xdotPrice > 0, "Invalid xDOT price");

        uint256 tokensToMint = ethAmount.mul(1 ether).div(xdotPrice);

        require(tokensToMint > 0, "Invalid mint amount");

        // Transfer the ETH to the t3rnVault.
        payable(t3rnVault).transfer(ethAmount);

        // Mint the xDOT tokens to the sender.
        _mint(msg.sender, tokensToMint);

        // Burn the aTRN tokens from the attesters.
        verifier.burnAttestedCollateral("xDOT", ethAmount);
    }

    function bridge(uint256 ethAmount) external payable {
        require(ethAmount <= verifier.getAttestedEthCollateral(), "Insufficient collateral");

        require(msg.value == ethAmount, "Mismatched ETH amount");
        uint256 xdotPrice = verifier.getPrice("xDOT");
        require(xdotPrice > 0, "Invalid xDOT price");

        uint256 tokensToMint = ethAmount.mul(1 ether).div(xdotPrice);

        require(tokensToMint > 0, "Invalid mint amount");

        // Transfer the ETH to the t3rnVault.
        payable(t3rnVault).transfer(ethAmount);

        // Mint the xDOT tokens to the sender.
        _mint(t3rnVault, tokensToMint);

        // Burn the aTRN tokens from the attesters.
        emit AttestedBridgeOrder(msg.sender, "xDOT", tokensToMint);
    }

    // This function allows you to update the verifier contract in case you upgrade or change it.
    function setVerifier(address _newVerifier) external onlyOwner {
        verifier = AttestationVerifier(_newVerifier);
    }
}


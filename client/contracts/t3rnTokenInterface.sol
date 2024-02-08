// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.20;

/// @dev The T3rn Token Precompile contract's address.
address constant T3RN_TOKEN_PRECOMPILE_ADDRESS = 0x0909090909090909090909090909090909090909;

/// @dev The T3rn Token Precompile contract's instance.
T3rnToken constant T3RN_TOKEN_CONTRACT = T3rnToken(T3RN_TOKEN_PRECOMPILE_ADDRESS);

/// @title The T3rn Token Precompile Interface
/// @dev The interface through which solidity contracts will interact with pallet-assets & pallet_balances.
/// @custom:address 0x0909090909090909090909090909090909090909
interface T3rnToken {
    /// @dev Gets the total supply of a currency.
    /// @return An uint256 representing the total supply of a token.
    function totalSupply() external view returns (uint256);

    /// @dev Gets balance of an address.
    /// @param owner address The address that owns a token.
    /// @return An uint256 representing the balance of the owner.
    function balanceOf(address owner) external view returns (uint256);

    /// @dev Gets the token  allowance of an address.
    /// @param owner address The address that owns a token
    /// @param token address The token address
    /// @return An uint256 representing of the token for the owner.
    function allowance(address owner, address token) external view returns (uint256);

    /// @dev Gets the name of a token.
    /// @return A bytes32 array representing the name of a token.
    function name() external view returns (bytes32);

    /// @dev Gets the symbol of a token.
    /// @return A bytes32 array representing the symbol of a token.
    function symbol() external view returns (bytes32);

    /// @dev Gets the decimals of a token.
    /// @return An uint256 representing the decimals of a token.
    function decimals() external view returns (uint256);

    /// @dev Transfer token to a specified address
    /// @param receiver address The address that will receive the token.
    /// @param value uint256 The value that will be transferred.
    /// @return true if the transfer was successful, revert otherwise
    function transfer(address receiver, uint256 value) external returns (bool);

    /// @dev Approve token for transfer.
    /// @param spender The token spender address.
    /// @param value uint256 The value that will be approved.
    /// @return true if the approval was successful, revert otherwise.
    function approve(address spender, uint256 value) external returns (bool);

    /// @dev Transfer token from a specified address to another one.
    /// @param sender The token sender address.
    /// @param receiver The token receiver address.
    /// @param value uint256 The value that will be transferred.
    /// @return true if the transfer was successful, revert otherwise.
    function transferFrom(address sender, address receiver, uint256 value) external returns (bool);

    /// @dev Event emitted when a token is transferred.
    /// @param sender address The address that transferred the token.
    /// @param receiver address The address that received the token.
    /// @param value uint256 The value that was transferred.
    event Transfer(address sender, address receiver, uint256 value);

    /// @dev Event emitted when a token is approved.
    /// @param spender The token spender address.
    /// @param value uint256 The value that was approved.
    event Approval(address indexed owner, address indexed spender, uint256 value);
}
#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::vec::*;
use t3rn_primitives::transfers::TransferEntry;
use t3rn_primitives::GatewayType;

use super::circuit_outbound::CircuitOutboundMessage;

pub trait GatewayInboundProtocol {
    /// Get storage on foreign chain under given key. Returns (gets it delivered by relayers):
    /// storage_value of storage
    /// storage_proof - Merkle Path and block reference securing against data collusion
    fn get_storage(&self, key: &[u8; 32], gateway_type: GatewayType) -> CircuitOutboundMessage;

    /// Set storage on foreign chain of given key pointing to new value. Returns (gets it delivered by relayers):
    /// new_storage_value of storage
    /// storage_proof - Merkle Path and block reference securing against data collusion
    /// finality_proof - Proof that the block is finalized
    fn set_storage(
        &self,
        key: &[u8; 32],
        value: Option<Vec<u8>>,
        gateway_type: GatewayType,
    ) -> CircuitOutboundMessage;

    /// Call smart contract behind that gateway in a static way - enforcing read-pnly operations. Returns (gets it delivered by relayers):
    /// stamp - Event generated on that chain
    /// stamp_proof - Merkle Path in storage trie (that's where the stamp lands) and block reference
    fn call_static(
        &self,
        module_name: &str,
        fn_name: &str,
        data: Vec<u8>,
        to: [u8; 32],
        value: u128,
        gas: u64,
        gateway_type: GatewayType,
    ) -> CircuitOutboundMessage;

    /// Call smart contract behind that gateway in a dirty way - without possibility to revert that action. Returns (gets it delivered by relayers):
    /// stamp - Event generated on that chain
    /// stamp_proof - Merkle Path in storage trie (that's where the stamp lands) and block reference
    /// finality_proof - Proof that the block is finalized
    fn call(
        &self,
        module_name: Vec<u8>,
        fn_name: Vec<u8>,
        data: Vec<u8>,
        escrow_account: [u8; 32],
        requester: [u8; 32],
        to: [u8; 32],
        value: u128,
        gas: u64,
        gateway_type: GatewayType,
    ) -> CircuitOutboundMessage;

    /// Call smart contract behind that gateway in a dirty way - without possibility to revert that action. Returns (gets it delivered by relayers):
    /// stamp - Event generated on that chain
    /// stamp_proof - Merkle Path in storage trie (that's where the stamp lands) and block reference
    /// finality_proof - Proof that the block is finalized
    fn call_dirty(
        &self,
        module_name: &str,
        fn_name: &str,
        data: Vec<u8>,
        to: [u8; 32],
        value: u128,
        gas: u64,
        gateway_type: GatewayType,
    ) -> CircuitOutboundMessage;

    /// Call smart contract behind that gateway in a reversible (escrowed) way. Returns (gets it delivered by relayers):
    /// stamp - Event generated on that chain
    /// stamp_proof - Merkle Path in storage trie (that's where the stamp lands) and block reference
    /// finality_proof - Proof that the block is finalized
    fn call_escrow(
        &self,
        module_name: &str,
        fn_name: &str,
        data: Vec<u8>,
        to: [u8; 32],
        value: u128,
        gas: u64,
        gateway_type: GatewayType,
    ) -> CircuitOutboundMessage;

    /// Call custom dispatchable call behind that gateway in a static way - enforcing read-pnly operations. Returns (gets it delivered by relayers):
    /// stamp - Event generated on that chain
    /// stamp_proof - Merkle Path in storage trie (that's where the stamp lands) and block reference
    fn custom_call_static(
        &self,
        module_name: &str,
        fn_name: &str,
        data: Vec<u8>,
        to: [u8; 32],
        value: u128,
        gas: u64,
        gateway_type: GatewayType,
    ) -> CircuitOutboundMessage;

    /// Call custom dispatchable call behind that gateway in a dirty way - without possibility to revert that action. Returns (gets it delivered by relayers):
    /// stamp - Event generated on that chain
    /// stamp_proof - Merkle Path in storage trie (that's where the stamp lands) and block reference
    /// finality_proof - Proof that the block is finalized
    fn custom_call_dirty(
        &self,
        module_name: &str,
        fn_name: &str,
        data: Vec<u8>,
        to: [u8; 32],
        value: u128,
        gas: u64,
        gateway_type: GatewayType,
    ) -> CircuitOutboundMessage;

    /// Call custom dispatchable call behind that gateway in a reversible (escrowed) way - enforcing read-pnly operations. Returns (gets it delivered by relayers):
    /// stamp - Event generated on that chain
    /// stamp_proof - Merkle Path in storage trie (that's where the stamp lands) and block reference
    /// finality_proof - Proof that the block is finalized
    fn custom_call_escrow(
        &self,
        module_name: &str,
        fn_name: &str,
        data: Vec<u8>,
        to: [u8; 32],
        value: u128,
        gas: u64,
        gateway_type: GatewayType,
    ) -> CircuitOutboundMessage;

    /// Transfer balance on a chain behind that gateway in a dirty way - without possibility to revert that action. Returns (gets it delivered by relayers):
    /// stamp - Event generated on that chain
    /// stamp_proof - Merkle Path in storage trie (that's where the stamp lands) and block reference
    /// finality_proof - Proof that the block is finalized
    fn transfer(
        &self,
        escrow_account: [u8; 32],
        requester: [u8; 32],
        to: [u8; 32],
        value: u128,
        transfers: &mut Vec<TransferEntry>,
        gateway_type: GatewayType,
    ) -> CircuitOutboundMessage;

    /// Transfer balance on a chain behind that gateway in a dirty way - without possibility to revert that action. Returns (gets it delivered by relayers):
    /// stamp - Event generated on that chain
    /// stamp_proof - Merkle Path in storage trie (that's where the stamp lands) and block reference
    /// finality_proof - Proof that the block is finalized
    fn transfer_dirty(
        &self,
        to: [u8; 32],
        value: u128,
        gas: u64,
        gateway_type: GatewayType,
    ) -> CircuitOutboundMessage;

    /// Transfer balance on a chain behind that gateway in a reversible (escrowed) way. Returns (gets it delivered by relayers):
    /// stamp - Event generated on that chain
    /// stamp_proof - Merkle Path in storage trie (that's where the stamp lands) and block reference
    /// finality_proof - Proof that the block is finalized
    fn transfer_escrow(
        &self,
        to: [u8; 32],
        value: u128,
        gas: u64,
        gateway_type: GatewayType,
    ) -> CircuitOutboundMessage;

    /// Swap X tokens to Y different tokens on a chain behind that gateway in a dirty way - without possibility to revert that action. Returns (gets it delivered by relayers):
    /// stamp - Event generated on that chain
    /// stamp_proof - Merkle Path in storage trie (that's where the stamp lands) and block reference
    /// finality_proof - Proof that the block is finalized
    fn swap_dirty(
        &self,
        to: [u8; 32],
        value: u128,
        gas: u64,
        gateway_type: GatewayType,
    ) -> CircuitOutboundMessage;

    /// Swap X tokens to Y different tokens on a chain behind that gateway in a reversible (escrowed) way. Returns (gets it delivered by relayers):
    /// stamp - Event generated on that chain
    /// stamp_proof - Merkle Path in storage trie (that's where the stamp lands) and block reference
    /// finality_proof - Proof that the block is finalized
    fn swap_escrow(
        &self,
        from: [u8; 32],
        x_token: [u8; 32],
        y_token: [u8; 32],
        x_value: u128,
        y_value: u128,
        gas: u64,
        gateway_type: GatewayType,
    ) -> CircuitOutboundMessage;
}

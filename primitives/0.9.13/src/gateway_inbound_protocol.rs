#![cfg_attr(not(feature = "std"), no_std)]

use crate::transfers::TransferEntry;
use crate::*;
use codec::Compact;
use sp_std::vec::Vec;

pub trait GatewayInboundProtocol {
    /// Get storage on foreign chain under given key. Returns (gets it delivered by relayers):
    /// storage_value of storage
    /// storage_proof - Merkle Path and block reference securing against data collusion
    fn get_storage(
        &self,
        key: Vec<u8>,
        gateway_type: GatewayType,
    ) -> Result<CircuitOutboundMessage, &'static str>;

    /// Set storage on foreign chain of given key pointing to new value. Returns (gets it delivered by relayers):
    /// new_storage_value of storage
    /// storage_proof - Merkle Path and block reference securing against data collusion
    /// finality_proof - Proof that the block is finalized
    fn set_storage(
        &self,
        key: Vec<u8>,
        value: Option<Vec<u8>>,
        gateway_type: GatewayType,
    ) -> Result<CircuitOutboundMessage, &'static str>;

    /// Call smart contract behind that gateway in a static way - enforcing read-pnly operations. Returns (gets it delivered by relayers):
    /// stamp - Event generated on that chain
    /// stamp_proof - Merkle Path in storage trie (that's where the stamp lands) and block reference
    fn call_static(
        &self,
        module_name: &str,
        fn_name: &str,
        data: Vec<u8>,
        to: Vec<u8>,
        value: Vec<u8>,
        gas: Vec<u8>,
        gateway_type: GatewayType,
        return_value: Option<Vec<u8>>,
    ) -> Result<CircuitOutboundMessage, &'static str>;

    /// Call smart contract behind that gateway in a dirty way - without possibility to revert that action. Returns (gets it delivered by relayers):
    /// stamp - Event generated on that chain
    /// stamp_proof - Merkle Path in storage trie (that's where the stamp lands) and block reference
    /// finality_proof - Proof that the block is finalized
    fn call(
        &self,
        module_name: Vec<u8>,
        fn_name: Vec<u8>,
        data: Vec<u8>,
        escrow_account: Vec<u8>,
        requester: Vec<u8>,
        to: Vec<u8>,
        value: Vec<u8>,
        gas: Vec<u8>,
        gateway_type: GatewayType,
        return_value: Option<Vec<u8>>,
    ) -> Result<CircuitOutboundMessage, &'static str>;

    /// Call smart contract behind that gateway in a reversible (escrowed) way. Returns (gets it delivered by relayers):
    /// stamp - Event generated on that chain
    /// stamp_proof - Merkle Path in storage trie (that's where the stamp lands) and block reference
    /// finality_proof - Proof that the block is finalized
    fn call_escrow(
        &self,
        module_name: &str,
        fn_name: &str,
        data: Vec<u8>,
        to: Vec<u8>,
        value: Vec<u8>,
        gas: Vec<u8>,
        gateway_type: GatewayType,
    ) -> Result<CircuitOutboundMessage, &'static str>;

    /// Call custom dispatchable call behind that gateway in a static way - enforcing read-pnly operations. Returns (gets it delivered by relayers):
    /// stamp - Event generated on that chain
    /// stamp_proof - Merkle Path in storage trie (that's where the stamp lands) and block reference
    fn custom_call_static(
        &self,
        module_name: &str,
        fn_name: &str,
        data: Vec<u8>,
        to: Vec<u8>,
        value: Vec<u8>,
        gas: Vec<u8>,
        gateway_type: GatewayType,
        return_value: Option<Vec<u8>>,
    ) -> Result<CircuitOutboundMessage, &'static str>;

    /// Call custom dispatchable call behind that gateway in a dirty way - without possibility to revert that action. Returns (gets it delivered by relayers):
    /// stamp - Event generated on that chain
    /// stamp_proof - Merkle Path in storage trie (that's where the stamp lands) and block reference
    /// finality_proof - Proof that the block is finalized
    fn custom_call_dirty(
        &self,
        module_name: &str,
        fn_name: &str,
        data: Vec<u8>,
        to: Vec<u8>,
        value: Vec<u8>,
        gas: Vec<u8>,
        gateway_type: GatewayType,
        return_value: Option<Vec<u8>>,
    ) -> Result<CircuitOutboundMessage, &'static str>;

    /// Call custom dispatchable call behind that gateway in a reversible (escrowed) way - enforcing read-pnly operations. Returns (gets it delivered by relayers):
    /// stamp - Event generated on that chain
    /// stamp_proof - Merkle Path in storage trie (that's where the stamp lands) and block reference
    /// finality_proof - Proof that the block is finalized
    fn custom_call_escrow(
        &self,
        module_name: &str,
        fn_name: &str,
        data: Vec<u8>,
        to: Vec<u8>,
        value: Vec<u8>,
        gas: Vec<u8>,
        gateway_type: GatewayType,
        return_value: Option<Vec<u8>>,
    ) -> Result<CircuitOutboundMessage, &'static str>;

    /// Transfer balance on a chain behind that gateway in a reversible (escrowed) way. Returns (gets it delivered by relayers):
    /// stamp - Event generated on that chain
    /// stamp_proof - Merkle Path in storage trie (that's where the stamp lands) and block reference
    /// finality_proof - Proof that the block is finalized
    fn transfer(
        &self,
        to: GenericAddress,
        value: Compact<u128>,
        gateway_type: GatewayType,
    ) -> Result<CircuitOutboundMessage, &'static str>;

    /// Transfer balance on a chain behind that gateway in a dirty way - without possibility to revert that action. Returns (gets it delivered by relayers):
    /// stamp - Event generated on that chain
    /// stamp_proof - Merkle Path in storage trie (that's where the stamp lands) and block reference
    /// finality_proof - Proof that the block is finalized
    fn transfer_escrow(
        &self,
        escrow_account: Vec<u8>,
        requester: Vec<u8>,
        to: Vec<u8>,
        value: Vec<u8>,
        transfers: &mut Vec<TransferEntry>,
        gateway_type: GatewayType,
    ) -> Result<CircuitOutboundMessage, &'static str>;

    /// Swap X tokens to Y different tokens on a chain behind that gateway in a dirty way - without possibility to revert that action. Returns (gets it delivered by relayers):
    /// stamp - Event generated on that chain
    /// stamp_proof - Merkle Path in storage trie (that's where the stamp lands) and block reference
    /// finality_proof - Proof that the block is finalized
    fn swap_dirty(
        &self,
        to: Vec<u8>,
        value: Vec<u8>,
        gas: Vec<u8>,
        gateway_type: GatewayType,
    ) -> Result<CircuitOutboundMessage, &'static str>;

    /// Swap X tokens to Y different tokens on a chain behind that gateway in a reversible (escrowed) way. Returns (gets it delivered by relayers):
    /// stamp - Event generated on that chain
    /// stamp_proof - Merkle Path in storage trie (that's where the stamp lands) and block reference
    /// finality_proof - Proof that the block is finalized
    fn swap_escrow(
        &self,
        from: Vec<u8>,
        x_token: Vec<u8>,
        y_token: Vec<u8>,
        x_value: Vec<u8>,
        y_value: Vec<u8>,
        gas: Vec<u8>,
        gateway_type: GatewayType,
    ) -> Result<CircuitOutboundMessage, &'static str>;
}

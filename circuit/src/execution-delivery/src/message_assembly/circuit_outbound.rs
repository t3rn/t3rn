#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Compact, Encode, Decode, alloc::collections::HashMap};

use crate::pallet::Config as Config;

use bp_messages::LaneId;






// #[cfg(feature = "std")]
// #[derive(Clone)]
// pub struct Api<P>
//     where
//         P: Pair,
//         MultiSignature: From<P::Signature>,
// {
//     pub url: String,
//     pub signer: Option<P>,
//     pub genesis_hash: Hash,
//     pub metadata: Metadata,
//     pub runtime_version: RuntimeVersion,
// }
//
// #[cfg(feature = "std")]
// impl<P> Api<P>
//     where
//         P: Pair,
//         MultiSignature: From<P::Signature>,
// {
//
// }

// #[derive(Debug, PartialEq, Eq)]
// pub struct CircuitValidatorSigningParams<Pair> {
//     // signer: String,
//     // signer_password: String,
//     secret: Pair,
// }
//
// impl CircuitValidatorSigningParams<Pair> {
//     pub fn new<Pair: sp_core::crypto::Pair>(seed: &str) -> &Self {
//         let pair = Pair::from_string(&format!("//{}", seed), None)
//             .expect("static values are valid; qed");
//
//         &CircuitValidatorSigningParams::<Pair> {
//             secret: pair
//         }
//     }
// }


/// CircuitOutbound covers the path of message assembly and adds it to the queue dispatchable by
pub enum CircuitOutbound<T: Config> {
    Programmable {
        escrow_account: T::AccountId,
        target_account: T::AccountId,
        message: Vec<u8>,
        gateway_id: bp_runtime::InstanceId,
        // secret_signing: CircuitValidatorSigningParams<sp_core::sr25519::Pair>,
    },
    TxOnlyExternal {
        escrow_account: T::AccountId,
        target_account: T::AccountId,
        message: Vec<u8>,
        gateway_id: bp_runtime::InstanceId,
    },
}

type Bytes = Vec<u8>;

/// Inclusion proofs of different tries
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub enum InboundStepProofTypes {
    /// Proof is a merkle path in the state trie
    State,
    /// Proof is a merkle path in the transaction trie (extrisics in Substrate)
    Transaction,
    /// Proof is a merkle path in the logs trie (in Substrate logs are entries in state trie)
    Logs,
}

/// Inbound Steps that specifie expected data deposited by relayers back to the Circuit after each step
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct CircuitInboundResult {
    pub result_format: Bytes,
    pub proof_type: InboundStepProofTypes,
}

/// Outbound Step that specifies expected transmission medium for relayers connecting with that gateway.
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub enum MessageTransmissionMedium {
    /// Request compatible with JSON-RPC API of receiving node
    Rpc {
        /// Encoded utf-8 string of module name that implements requested entrypoint
        module_name: Bytes,
        /// Encoded utf-8 string of method name that implements requested entrypoint
        method_name: Bytes,
    },
    /// Transaction (in substrate extrinics), signed offline and including dispatch call(s)
    TransactionDispatch {
        /// Encoded call bytes
        call_bytes: Bytes,
        /// Encoded tx signature
        signature: Bytes,
        /// Encoded extras to that transctions, like versions and gas price /tips for miners. Check GenericExtra for more info.
        extra: Bytes,
    },
    /// Custom transmission medium (like Substrate's XCMP)
    Custom {
        /// Custom message bytes, that would have to be decoded by the receiving end.
        payload: Bytes,
    },
}

/// Outbound Step that specifies expected transmission medium for relayers connecting with that gateway.
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub enum CircuitOutboundMessage {
    /// Request compatible with JSON-RPC API of receiving node
    Read {
        /// Array of next arguments: encoded bytes of arguments that that JSON-RPC API expects
        arguments: Vec<Bytes>,
        /// Expected results that will be decoded and checked against the format
        inbound_results: CircuitInboundResult,
        /// Expected results
        transmission_medium: MessageTransmissionMedium,
    },
    /// Transaction (in substrate extrinics), signed offline and including dispatch call(s)
    Write {
        /// Encoded sender's public key
        sender: Bytes,
        /// Array of next arguments: encoded bytes of arguments that that JSON-RPC API expects
        arguments: Vec<Bytes>,
        /// Expected results
        inbound_results: CircuitInboundResult,
        /// Expected results
        transmission_medium: MessageTransmissionMedium,
    },
    /// Custom transmission medium (like Substrate's XCMP)
    Escrowed {
        /// Encoded sender's public key
        sender: Bytes,
        /// Encoded target's public key
        target: Bytes,
        /// Array of next arguments: encoded bytes of arguments that that JSON-RPC API expects
        arguments: Vec<Bytes>,
        /// Expected results
        inbound_results: CircuitInboundResult,
        /// Expected results
        transmission_medium: MessageTransmissionMedium,
    }
}

pub trait CircuitOutboundProtocol {

}


impl<T: Config> CircuitOutbound<T>  {

    // fn sign_message(&self, message: T::OutboundPayload, submitter: T::AccountId, gateway_inbound: &GatewayAssembly) -> Vec<u8> {
    //     let signed_message = match self {
    //         // CircuitOutbound::ProgrammableInternal { message, .. } => {
    //         //     // In case of Internal Programmable Gateway rely fully on pallet-bridge-messages
    //         //     // Some entity needs to sign transaction here in the way recognizable by send_message + call_dispatch implementations
    //         //     // gateway_runtime::MessagesCall::send_message(
    //         //     //     lane, message, fee,
    //         //     // )
    //         //
    //         //     let number: T::BlockNumber = Zero::zero();
    //         //     // let parent_hash = <system::Pallet<T>>::block_hash(block_number - 1u32.into());
    //         //     let genesis_hash = <frame_system::Pallet<T>>::block_hash(Zero::zero());
    //         //
    //         //     Circuit::sign_transaction(
    //         //         genesis_hash,
    //         //         &source_sign,
    //         //         transaction_nonce,
    //         //         send_message_call,
    //         //     );
    //         //     vec![]
    //         // },
    //         CircuitOutbound::Programmable { escrow_account, target_account, message, gateway_id, secret_signing } => {
    //             // Sign message as dispatchable xtrinsics with account that is alive & has positive balance on foreign chain.
    //
    //             /// Nonce of validator on foreign chain needs to be known here.
    //             /// For that validators can keep update data about their validators account on-chain
    //             /// and that data could be accessed here directly from storage of that module.
    //             let nonce = 0;
    //
    //             gateway_inbound.sign_call_offline(*message, secret_signing.secret, 0)
    //         },
    //         CircuitOutbound::TxOnlyExternal { escrow_account, target_account, message, gateway_id } => {
    //             // Sign message as a federated POA validators as PoC I & II and move to Fast Multiparty Threshold ECDSA in PoC III.
    //
    //             vec![]
    //         },
    //     };
    //     signed_message
    // }
    fn send_message(&self, message: T::OutboundPayload, submitter: T::AccountId) -> Vec<u8> {

        // Q: What's the best way to recognize the transmission medium for relayers now:
        // A1) have several dedicated lanes per each transmission medium:
        // bridge <-> bridge messages
        // ?dispatch calls
        // ?xcmp messages
        // ?rpc messages?

        // Additional format with multiple variants
            // XCM variant - XCM
                // GENERIC
            // Generic Dispatch
            // Custom Interaction
        // Blockchain vs

        // ACCESS LAYER
            // RPC
            // Runtime API - read-only - what's the validator set - network general data without
                // RPC state_call
                // Proof against next runtime
            // Runtime - directly via storage runtime
            // Dispatch is a subset of XCM
            //
        /**
            origin,
			lane_id: LaneId,
			payload: T::OutboundPayload,
			delivery_and_dispatch_fee: T::OutboundMessageFee,
        **/
        let origin = frame_system::RawOrigin::Signed(submitter).into();
        let lane_id: LaneId = [0, 0, 0, 1];
        let delivery_and_dispatch_fee: T::OutboundMessageFee = 0.into();

        <pallet_bridge_messages::Module<T>>::send_message(
            origin,
            lane_id,
            message,
            delivery_and_dispatch_fee,
        );

        vec![]

    }
}

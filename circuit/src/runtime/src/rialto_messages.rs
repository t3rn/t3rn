// Copyright 2019-2020 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! Everything required to serve Circuit <-> Rialto messages.

use crate::Runtime;

use bp_messages::{
	source_chain::TargetHeaderChain,
	target_chain::{ProvedMessages, SourceHeaderChain},
	InboundLaneData, LaneId, Message, MessageNonce, Parameter as MessagesParameter,
};
use bp_runtime::{InstanceId, RIALTO_BRIDGE_INSTANCE};
use bridge_runtime_common::messages::{self, ChainWithMessages, MessageBridge, MessageTransaction};
use codec::{Decode, Encode};
use frame_support::{
	parameter_types,
	weights::{DispatchClass, Weight},
	RuntimeDebug,
};
use sp_core::storage::StorageKey;
use sp_runtime::{FixedPointNumber, FixedU128};
use sp_std::{convert::TryFrom, ops::RangeInclusive};

parameter_types! {
	/// Rialto to Circuit conversion rate. Initially we treat both tokens as equal.
	storage RialtoToCircuitConversionRate: FixedU128 = FixedU128::one();
}

/// Storage key of the Circuit -> Rialto message in the runtime storage.
pub fn message_key(lane: &LaneId, nonce: MessageNonce) -> StorageKey {
	pallet_bridge_messages::storage_keys::message_key::<Runtime, <Circuit as ChainWithMessages>::MessagesInstance>(
		lane, nonce,
	)
}

/// Storage key of the Circuit -> Rialto message lane state in the runtime storage.
pub fn outbound_lane_data_key(lane: &LaneId) -> StorageKey {
	pallet_bridge_messages::storage_keys::outbound_lane_data_key::<<Circuit as ChainWithMessages>::MessagesInstance>(
		lane,
	)
}

/// Storage key of the Rialto -> Circuit message lane state in the runtime storage.
pub fn inbound_lane_data_key(lane: &LaneId) -> StorageKey {
	pallet_bridge_messages::storage_keys::inbound_lane_data_key::<
		Runtime,
		<Circuit as ChainWithMessages>::MessagesInstance,
	>(lane)
}

/// Message payload for Circuit -> Rialto messages.
pub type ToRialtoMessagePayload = messages::source::FromThisChainMessagePayload<WithRialtoMessageBridge>;

/// Message verifier for Circuit -> Rialto messages.
pub type ToRialtoMessageVerifier = messages::source::FromThisChainMessageVerifier<WithRialtoMessageBridge>;

/// Message payload for Rialto -> Circuit messages.
pub type FromRialtoMessagePayload = messages::target::FromBridgedChainMessagePayload<WithRialtoMessageBridge>;

/// Encoded Circuit Call as it comes from Rialto.
pub type FromRialtoEncodedCall = messages::target::FromBridgedChainEncodedMessageCall<WithRialtoMessageBridge>;

/// Messages proof for Rialto -> Circuit messages.
type FromRialtoMessagesProof = messages::target::FromBridgedChainMessagesProof<bp_rialto::Hash>;

/// Messages delivery proof for Circuit -> Rialto messages.
type ToRialtoMessagesDeliveryProof = messages::source::FromBridgedChainMessagesDeliveryProof<bp_rialto::Hash>;

/// Call-dispatch based message dispatch for Rialto -> Circuit messages.
pub type FromRialtoMessageDispatch = messages::target::FromBridgedChainMessageDispatch<
	WithRialtoMessageBridge,
	crate::Runtime,
	pallet_bridge_dispatch::DefaultInstance,
>;

/// Circuit <-> Rialto message bridge.
#[derive(RuntimeDebug, Clone, Copy)]
pub struct WithRialtoMessageBridge;

impl MessageBridge for WithRialtoMessageBridge {
	const INSTANCE: InstanceId = RIALTO_BRIDGE_INSTANCE;

	const RELAYER_FEE_PERCENT: u32 = 10;

	type ThisChain = Circuit;
	type BridgedChain = Rialto;

	fn bridged_balance_to_this_balance(bridged_balance: bp_rialto::Balance) -> bp_circuit::Balance {
		bp_circuit::Balance::try_from(RialtoToCircuitConversionRate::get().saturating_mul_int(bridged_balance))
			.unwrap_or(bp_circuit::Balance::MAX)
	}
}

/// Circuit chain from message lane point of view.
#[derive(RuntimeDebug, Clone, Copy)]
pub struct Circuit;

impl messages::ChainWithMessages for Circuit {
	type Hash = bp_circuit::Hash;
	type AccountId = bp_circuit::AccountId;
	type Signer = bp_circuit::AccountSigner;
	type Signature = bp_circuit::Signature;
	type Weight = Weight;
	type Balance = bp_circuit::Balance;

	type MessagesInstance = pallet_bridge_messages::DefaultInstance;
}

impl messages::ThisChainWithMessages for Circuit {
	type Call = crate::Call;

	fn is_outbound_lane_enabled(lane: &LaneId) -> bool {
		*lane == LaneId::default()
	}

	fn maximal_pending_messages_at_outbound_lane() -> MessageNonce {
		MessageNonce::MAX
	}

	fn estimate_delivery_confirmation_transaction() -> MessageTransaction<Weight> {
		let inbound_data_size =
			InboundLaneData::<bp_circuit::AccountId>::encoded_size_hint(bp_circuit::MAXIMAL_ENCODED_ACCOUNT_ID_SIZE, 1)
				.unwrap_or(u32::MAX);

		MessageTransaction {
			dispatch_weight: bp_circuit::MAX_SINGLE_MESSAGE_DELIVERY_CONFIRMATION_TX_WEIGHT,
			size: inbound_data_size
				.saturating_add(bp_rialto::EXTRA_STORAGE_PROOF_SIZE)
				.saturating_add(bp_circuit::TX_EXTRA_BYTES),
		}
	}

	fn transaction_payment(transaction: MessageTransaction<Weight>) -> bp_circuit::Balance {
		// in our testnets, both per-byte fee and weight-to-fee are 1:1
		messages::transaction_payment(
			bp_circuit::BlockWeights::get().get(DispatchClass::Normal).base_extrinsic,
			1,
			FixedU128::zero(),
			|weight| weight as _,
			transaction,
		)
	}
}

/// Rialto chain from message lane point of view.
#[derive(RuntimeDebug, Clone, Copy)]
pub struct Rialto;

impl messages::ChainWithMessages for Rialto {
	type Hash = bp_rialto::Hash;
	type AccountId = bp_rialto::AccountId;
	type Signer = bp_rialto::AccountSigner;
	type Signature = bp_rialto::Signature;
	type Weight = Weight;
	type Balance = bp_rialto::Balance;

	type MessagesInstance = pallet_bridge_messages::DefaultInstance;
}

impl messages::BridgedChainWithMessages for Rialto {
	fn maximal_extrinsic_size() -> u32 {
		bp_rialto::max_extrinsic_size()
	}

	fn message_weight_limits(_message_payload: &[u8]) -> RangeInclusive<Weight> {
		// we don't want to relay too large messages + keep reserve for future upgrades
		let upper_limit = messages::target::maximal_incoming_message_dispatch_weight(bp_rialto::max_extrinsic_weight());

		// we're charging for payload bytes in `WithRialtoMessageBridge::transaction_payment` function
		//
		// this bridge may be used to deliver all kind of messages, so we're not making any assumptions about
		// minimal dispatch weight here

		0..=upper_limit
	}

	fn estimate_delivery_transaction(
		message_payload: &[u8],
		message_dispatch_weight: Weight,
	) -> MessageTransaction<Weight> {
		let message_payload_len = u32::try_from(message_payload.len()).unwrap_or(u32::MAX);
		let extra_bytes_in_payload = Weight::from(message_payload_len)
			.saturating_sub(pallet_bridge_messages::EXPECTED_DEFAULT_MESSAGE_LENGTH.into());

		MessageTransaction {
			dispatch_weight: extra_bytes_in_payload
				.saturating_mul(bp_rialto::ADDITIONAL_MESSAGE_BYTE_DELIVERY_WEIGHT)
				.saturating_add(bp_rialto::DEFAULT_MESSAGE_DELIVERY_TX_WEIGHT)
				.saturating_add(message_dispatch_weight),
			size: message_payload_len
				.saturating_add(bp_circuit::EXTRA_STORAGE_PROOF_SIZE)
				.saturating_add(bp_rialto::TX_EXTRA_BYTES),
		}
	}

	fn transaction_payment(transaction: MessageTransaction<Weight>) -> bp_rialto::Balance {
		// in our testnets, both per-byte fee and weight-to-fee are 1:1
		messages::transaction_payment(
			bp_rialto::BlockWeights::get().get(DispatchClass::Normal).base_extrinsic,
			1,
			FixedU128::zero(),
			|weight| weight as _,
			transaction,
		)
	}
}

impl TargetHeaderChain<ToRialtoMessagePayload, bp_rialto::AccountId> for Rialto {
	type Error = &'static str;
	// The proof is:
	// - hash of the header this proof has been created with;
	// - the storage proof or one or several keys;
	// - id of the lane we prove state of.
	type MessagesDeliveryProof = ToRialtoMessagesDeliveryProof;

	fn verify_message(payload: &ToRialtoMessagePayload) -> Result<(), Self::Error> {
		messages::source::verify_chain_message::<WithRialtoMessageBridge>(payload)
	}

	fn verify_messages_delivery_proof(
		proof: Self::MessagesDeliveryProof,
	) -> Result<(LaneId, InboundLaneData<bp_circuit::AccountId>), Self::Error> {
		messages::source::verify_messages_delivery_proof::<WithRialtoMessageBridge, Runtime>(proof)
	}
}

impl SourceHeaderChain<bp_rialto::Balance> for Rialto {
	type Error = &'static str;
	// The proof is:
	// - hash of the header this proof has been created with;
	// - the storage proof or one or several keys;
	// - id of the lane we prove messages for;
	// - inclusive range of messages nonces that are proved.
	type MessagesProof = FromRialtoMessagesProof;

	fn verify_messages_proof(
		proof: Self::MessagesProof,
		messages_count: u32,
	) -> Result<ProvedMessages<Message<bp_rialto::Balance>>, Self::Error> {
		messages::target::verify_messages_proof::<WithRialtoMessageBridge, Runtime>(proof, messages_count)
	}
}

/// Circuit -> Rialto message lane pallet parameters.
#[derive(RuntimeDebug, Clone, Encode, Decode, PartialEq, Eq)]
pub enum CircuitToRialtoMessagesParameter {
	/// The conversion formula we use is: `CircuitTokens = RialtoTokens * conversion_rate`.
	RialtoToCircuitConversionRate(FixedU128),
}

impl MessagesParameter for CircuitToRialtoMessagesParameter {
	fn save(&self) {
		match *self {
			CircuitToRialtoMessagesParameter::RialtoToCircuitConversionRate(ref conversion_rate) => {
				RialtoToCircuitConversionRate::set(conversion_rate)
			}
		}
	}
}

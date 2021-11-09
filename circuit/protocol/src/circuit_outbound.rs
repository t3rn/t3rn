#![cfg_attr(not(feature = "std"), no_std)]

use bp_messages::LaneId;
use pallet_bridge_messages::Config;
use sp_std::vec;
use sp_std::vec::*;

/// CircuitOutbound covers the path of message assembly and adds it to the queue dispatchable by
pub enum CircuitOutbound<T: Config> {
    Programmable {
        escrow_account: T::AccountId,
        target_account: T::AccountId,
        message: Vec<u8>,
        gateway_id: bp_runtime::ChainId,
    },
    TxOnlyExternal {
        escrow_account: T::AccountId,
        target_account: T::AccountId,
        message: Vec<u8>,
        gateway_id: bp_runtime::ChainId,
    },
}

pub trait CircuitOutboundProtocol {}

impl<T: Config> CircuitOutbound<T> {
    pub fn send_message(&self, message: T::OutboundPayload, submitter: T::AccountId) -> Vec<u8> {
        let origin = frame_system::RawOrigin::Signed(submitter).into();
        let lane_id: LaneId = [0, 0, 0, 1];
        let delivery_and_dispatch_fee: T::OutboundMessageFee = 0.into();

        let _res = <pallet_bridge_messages::Module<T>>::send_message(
            origin,
            lane_id,
            message,
            delivery_and_dispatch_fee,
        );

        vec![]
    }
}

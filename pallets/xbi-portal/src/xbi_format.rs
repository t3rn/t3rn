use codec::{Decode, Encode};
use core::fmt::Debug;
use scale_info::TypeInfo;
use sp_runtime::AccountId32;
use xcm::latest::{Junction, MultiLocation};

pub type Bytes = Vec<u8>;
pub type AssetId = u64;
// pub type Balance16B = MultiAsset;
pub type Balance16B = u128;
pub type AccountIdOf = MultiLocation;
// pub type AccountId32 = AccountId32;
// pub type AccountKey20 = AccountKey20;

#[derive(Clone, Eq, PartialEq, Debug, Default, Encode, Decode, TypeInfo)]
pub struct XBIFormat {
    pub instr: XBIInstr,
    pub metadata: XBIMetadata,
}
#[derive(Clone, Eq, PartialEq, Debug, Encode, Decode, TypeInfo)]
pub enum XBIInstr {
    CallNative {
        payload: Bytes,
    },
    CallEvm {
        caller: AccountId32,
        dest: Junction, // Junction::AccountKey20
        value: Balance16B,
        input: Bytes,
        gas_limit: Balance16B,
        max_fee_per_gas: Option<Balance16B>,
        max_priority_fee_per_gas: Option<Balance16B>,
        nonce: Option<u32>,
        access_list: Option<Bytes>,
    },
    CallWasm {
        caller: AccountId32,
        dest: AccountId32,
        value: Balance16B,
        input: Bytes,
    },
    CallCustom {
        caller: AccountId32,
        dest: AccountId32,
        value: Balance16B,
        input: Bytes,
        additional_params: Option<Vec<Bytes>>,
    },
    Transfer {
        dest: AccountId32,
        value: Balance16B,
    },
    TransferMulti {
        currency_id: AssetId,
        dest: AccountId32,
        value: Balance16B,
    },
    Result {
        success: bool,
        output: Bytes,
        witness: Bytes,
    },
    Notification {
        kind: XBINotificationKind,
        instruction_id: Bytes,
        extra: Bytes,
    },
}

impl Default for XBIInstr {
    fn default() -> Self {
        XBIInstr::CallNative { payload: vec![] }
    }
}

pub type Timeout = u128;

#[derive(Clone, Eq, PartialEq, Debug, Encode, Decode, TypeInfo)]
pub enum XBINotificationKind {
    Sent,
    Delivered,
    Executed,
}
#[derive(Clone, Eq, PartialEq, Debug, Default, Encode, Decode, TypeInfo)]
pub struct ActionNotificationTimeouts {
    action: Timeout,
    notification: Timeout,
}

#[derive(Clone, Eq, PartialEq, Debug, Default, Encode, Decode, TypeInfo)]
pub struct XBIMetadata {
    pub sent: ActionNotificationTimeouts,
    pub delivered: ActionNotificationTimeouts,
    pub executed: ActionNotificationTimeouts,
}
// //   - `Sent (action timeout, notification timeout)`
// //   - `Delivered (action timeout, notification timeout)`
// //   - `Executed (action timeout, notification timeout)`
// //   - `Destination / Bridge security guarantees (e.g. in confirmation no for PoW, finality proofs)`
// //   - `max_exec_cost`: `Balance` : `Maximal cost / fees for execution of delivery`
// //   - `max_notification_cost`: `Balance` : `Maximal cost / fees per delivering notification`
//
// pub enum XBIMetadata {
// 	Sent { action: Timeout, notification: Timeout },
// 	Delivered { action: Timeout, notification: Timeout },
// 	Executed { action: Timeout, notification: Timeout },
// 	// //   - `Sent (action timeout, notification timeout)`
// 	// //   - `Delivered (action timeout, notification timeout)`
// 	// //   - `Executed (action timeout, notification timeout)`
// 	// //   - `Destination / Bridge security guarantees (e.g. in confirmation no for PoW, finality proofs)`
// 	// //   - `max_exec_cost`: `Balance` : `Maximal cost / fees for execution of delivery`
// 	// //   - `max_notification_cost`: `Balance` : `Maximal cost / fees per delivering notification`
// }

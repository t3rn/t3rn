use codec::{Decode, Encode};
use scale_info::TypeInfo;
#[cfg(feature = "no_std")]
use sp_runtime::RuntimeDebug as Debug;
#[cfg(feature = "std")]
use std::fmt::Debug;

use sp_std::{prelude::*, vec::Vec};
pub use t3rn_primitives::side_effect::{EventSignature, SideEffectId, SideEffectName};
use t3rn_primitives::xdns::XdnsRecord;

/// The object with XdnsRecords as returned by the RPC endpoint
#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct FetchXdnsRecordsResponse<AccountId> {
    pub xdns_records: Vec<XdnsRecord<AccountId>>,
}

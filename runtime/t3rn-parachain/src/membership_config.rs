use super::*;
use crate::DeveloperMembership;
use frame_system::{EnsureNever, EnsureRoot, EnsureSignedBy};
use sp_runtime::traits::ConstU32;

impl pallet_membership::Config for Runtime {
    type AddOrigin = EnsureSignedBy<DeveloperMembership, AccountId>;
    type Event = Event;
    type MaxMembers = ConstU32<64>;
    type MembershipChanged = ();
    type MembershipInitialized = ();
    type PrimeOrigin = EnsureRoot<AccountId>;
    type RemoveOrigin = EnsureRoot<AccountId>;
    type ResetOrigin = EnsureRoot<AccountId>;
    type SwapOrigin = EnsureNever<AccountId>;
    type WeightInfo = ();
}

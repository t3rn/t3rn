use crate::*;
use frame_support::traits::ConstU32;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
    use super::*;

    pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

    impl_opaque_keys! {
        pub struct SessionKeys {
            pub aura: Aura,
            pub grandpa: Grandpa,
        }
    }
}

impl pallet_aura::Config for Runtime {
    type AuthorityId = AuraId;
    type DisabledValidators = ();
    type MaxAuthorities = ConstU32<32>;
}

use pallet_grandpa::AuthorityId as GrandpaId;

impl pallet_grandpa::Config for Runtime {
    type Call = Call;
    type Event = Event;
    type HandleEquivocation = ();
    type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
        KeyTypeId,
        GrandpaId,
    )>>::IdentificationTuple;
    type KeyOwnerProof =
        <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::Proof;
    type KeyOwnerProofSystem = ();
    type MaxAuthorities = ConstU32<32>;
    type WeightInfo = ();
}

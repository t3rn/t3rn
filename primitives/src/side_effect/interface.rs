use crate::{
    abi::Type,
    protocol::SideEffectProtocol,
    side_effect::{EventSignature, SideEffectConfirmationProtocol, SideEffectName},
};
use codec::{Decode, Encode};
use scale_info::TypeInfo;
#[cfg(feature = "no_std")]
use sp_runtime::RuntimeDebug as Debug;
use sp_std::vec::*;
#[cfg(feature = "std")]
use std::fmt::Debug;

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo, Default)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct SideEffectInterface {
    pub id: [u8; 4],
    pub name: SideEffectName,
    pub argument_abi: Vec<Type>,
    pub argument_to_state_mapper: Vec<EventSignature>,
    pub confirm_events: Vec<EventSignature>,
    pub escrowed_events: Vec<EventSignature>,
    pub commit_events: Vec<EventSignature>,
    pub revert_events: Vec<EventSignature>,
}

impl SideEffectInterface {
    pub fn generate_id<Hasher: sp_core::Hasher>(&self) -> <Hasher as sp_core::Hasher>::Out {
        Hasher::hash(Encode::encode(self).as_ref())
    }
}

impl SideEffectProtocol for SideEffectInterface {
    fn get_id(&self) -> [u8; 4] {
        self.id
    }

    fn get_name(&self) -> SideEffectName {
        self.name.clone()
    }

    fn get_arguments_abi(&self) -> Vec<Type> {
        self.argument_abi.clone()
    }

    /// ToDo: Protocol::Reversible x-t3rn#69 - !Inspect if from is doable here - the original transfer is from a user,
    ///     whereas the transfers on targets are made by relayers/executors.
    ///     Prefer to only inspect the the target
    /// ToDo: Protocol::Reversible - Support optional parameters like insurance. - must be hardcoded name
    ///         // vec!["from", "to", "value", "Option<insurance>"]
    fn get_arguments_2_state_mapper(&self) -> Vec<EventSignature> {
        self.argument_to_state_mapper.clone()
    }

    fn get_confirming_events(&self) -> Vec<EventSignature> {
        self.confirm_events.clone()
    }

    /// This event must be emitted by Escrow Contracts
    fn get_escrowed_events(&self) -> Vec<EventSignature> {
        self.escrowed_events.clone()
    }

    fn get_reversible_commit(&self) -> Vec<EventSignature> {
        self.commit_events.clone()
    }

    /// ToDo: Protocol::Reversible x-t3rn#69 - If executors wants to avoid loosing their insurance deposit, must return the funds to the original user
    ///     - that's problematic since we don't know user's address on target
    ///     Temp. solution before the locks in wrapped tokens on Circuit is to leave it empty
    ///     and Commit relayer to perform the transfer for 100% or they loose insured deposit
    fn get_reversible_revert(&self) -> Vec<EventSignature> {
        self.revert_events.clone()
    }
}

impl SideEffectConfirmationProtocol for SideEffectInterface {}

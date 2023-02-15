use crate::{
    abi::Type,
    side_effect::{EventSignature, SideEffectName},
};
use scale_info::prelude::vec::Vec;

use codec::{Decode, Encode};

use scale_info::TypeInfo;

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

    pub fn get_id(&self) -> [u8; 4] {
        self.id
    }

    pub fn get_name(&self) -> SideEffectName {
        self.name.clone()
    }

    pub fn get_arguments_abi(&self) -> Vec<Type> {
        self.argument_abi.clone()
    }

    pub fn get_arguments_2_state_mapper(&self) -> Vec<EventSignature> {
        self.argument_to_state_mapper.clone()
    }

    pub fn get_confirming_events(&self) -> Vec<EventSignature> {
        self.confirm_events.clone()
    }

    pub fn get_escrowed_events(&self) -> Vec<EventSignature> {
        self.escrowed_events.clone()
    }

    pub fn get_reversible_commit(&self) -> Vec<EventSignature> {
        self.commit_events.clone()
    }

    pub fn get_reversible_revert(&self) -> Vec<EventSignature> {
        self.revert_events.clone()
    }
}

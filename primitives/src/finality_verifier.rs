use crate::{
    side_effect::{FullSideEffect, SideEffect},
    volatile::LocalState,
};
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::marker::PhantomData;

use sp_std::vec::Vec;

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct FinalityVerifierImpl<T: frame_system::Config> {
    _marker: PhantomData<T>,
}

pub trait FinalityVerifier<T: frame_system::Config> {
    fn verify_event_inclusion(
        &self,
        encoded_arguments: Vec<u8>
    ) -> Result<(), &'static str>;
    fn init_bridge(
        &self,
        encoded_arguments: Vec<u8>
    ) -> Result<(), &'static str>;
    fn decode_event_arguments(
        &self,
        encoded_arguments: Vec<u8>
    ) -> Result<(), &'static str>;
}

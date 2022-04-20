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
pub struct FreeVMImpl<T: frame_system::Config> {
    _marker: PhantomData<T>,
}

pub trait FreeVM<T: frame_system::Config> {
    fn exec_in_xtx_ctx(
        &self,
        xtx_id: T::Hash,
        local_state: LocalState,
        full_side_effects: Vec<Vec<FullSideEffect<T::AccountId, T::BlockNumber, u128>>>,
        steps_cnt: (u32, u32),
    ) -> Result<Vec<Vec<SideEffect<T::AccountId, T::BlockNumber, u128>>>, &'static str>;
}

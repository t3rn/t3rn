#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    dispatch::DispatchResultWithPostInfo,
    pallet_prelude::*,
    traits::{Currency, ReservableCurrency},
};

use frame_system::pallet_prelude::*;
pub use pallet::*;
use scale_info::TypeInfo;

use sp_std::{convert::TryInto, vec::Vec};
use t3rn_primitives::{
    circuit::{
        traits::CircuitSubmitAPI,
        types::{OrderSFX, SFXAction},
    },
    SpeedMode,
};
pub type Asset = u32;
pub type Destination = [u8; 4];
pub type Input = Vec<u8>;
use t3rn_abi::types::Sfx4bId;
use t3rn_primitives::circuit::SideEffect;

t3rn_primitives::reexport_currency_types!();

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
        type CircuitSubmitAPI: CircuitSubmitAPI<Self, BalanceOf<Self>>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // Define your events here
    }

    #[pallet::error]
    pub enum Error<T> {
        // Define your errors here
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn order(
            origin: OriginFor<T>,
            sfx_actions: Vec<
                OrderSFX<T::AccountId, Asset, BalanceOf<T>, Destination, Input, BalanceOf<T>>,
            >,
            speed_mode: SpeedMode,
        ) -> DispatchResultWithPostInfo {
            let side_effects: Vec<SideEffect<T::AccountId, BalanceOf<T>>> = sfx_actions
                .into_iter()
                .map(|sfx_action| sfx_action.try_into())
                .collect::<Result<Vec<SideEffect<T::AccountId, BalanceOf<T>>>, DispatchError>>()?;

            T::CircuitSubmitAPI::on_extrinsic_trigger(origin, side_effects, speed_mode)?;

            Ok(().into())
        }
    }
}

#[cfg(test)]
mod tests {
    use frame_support::assert_ok;
    use sp_runtime::AccountId32;
    use t3rn_mini_mock_runtime::{
        AccountId, Balance, Balances, Circuit, ExtBuilder, Origin, Vacuum,
    };
    use t3rn_primitives::{
        circuit::{
            traits::CircuitSubmitAPI,
            types::{OrderSFX, SFXAction},
        },
        SpeedMode,
    };

    pub type TargetId = [u8; 4];
    const ETHEREUM_TARGET: TargetId = [0u8; 4];
    const ASTAR_TARGET: TargetId = [8u8; 4];
    const POLKADOT_TARGET: TargetId = [1u8; 4];
    const KUSAMA_TARGET: TargetId = [2u8; 4];

    const TRN: Balance = 1_000_000_000_000u128;
    use frame_support::traits::Currency;

    #[test]
    fn order_single_sfx_vacuum_delivers_to_circuit() {
        let mut ext = ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_polkadot_gateway_record()
            .with_eth_gateway_record()
            .build();
        ext.execute_with(|| {
            // Load requester with some funds
            let requester = AccountId::from([1u8; 32]);
            Balances::deposit_creating(&requester, 100_000 * TRN);

            let sfx_action =
                SFXAction::Transfer(POLKADOT_TARGET, 1u32, AccountId32::new([2u8; 32]), 100u128);
            let sfx_order = OrderSFX::<AccountId32, u32, u128, [u8; 4], Vec<u8>, u128> {
                sfx_action,
                max_reward: 200u128,
                insurance: 50u128,
                reward_asset: 1u32,
            };
            let res = Vacuum::order(
                Origin::signed(requester.clone()),
                vec![sfx_order],
                SpeedMode::Fast,
            );

            assert_ok!(res);
        });
    }
}

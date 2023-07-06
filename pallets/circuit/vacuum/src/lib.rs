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
        prepare_ext_builder_playground, AccountId, Assets, Balance, Balances, Circuit, ExtBuilder,
        MiniRuntime, Origin, Vacuum, ASSET_DOT, ASSET_ETH, ASSET_KSM, ASSET_TRN, ASSET_USDT,
        ASTAR_TARGET, KUSAMA_TARGET, POLKADOT_TARGET, XDNS,
    };
    use t3rn_primitives::{
        circuit::{
            traits::CircuitSubmitAPI,
            types::{OrderSFX, SFXAction},
        },
        monetary::TRN,
        SpeedMode, TreasuryAccount, TreasuryAccountProvider,
    };

    use frame_support::traits::Currency;
    use t3rn_primitives::monetary::EXISTENTIAL_DEPOSIT;

    #[test]
    fn order_single_sfx_vacuum_delivers_to_circuit() {
        let mut ext = prepare_ext_builder_playground();
        ext.execute_with(|| {
            assert!(XDNS::all_token_ids().contains(&ASSET_DOT));
            // Load requester with some funds
            let requester = AccountId::from([1u8; 32]);
            let issuer_is_escrow_account =
                MiniRuntime::get_treasury_account(TreasuryAccount::Escrow);
            Balances::deposit_creating(&requester, (100_000 * TRN) as Balance); // To cover fees
            assert_ok!(ssets::mint(
                Origin::signed(issuer_is_escrow_account),
                ASSET_DOT,
                requester.clone(),
                200u128 + (EXISTENTIAL_DEPOSIT as Balance),
            ));
            assert_eq!(Assets::balance(ASSET_DOT, &requester), 201u128);

            let sfx_action =
                SFXAction::Transfer(POLKADOT_TARGET, 1u32, AccountId32::new([2u8; 32]), 100u128);
            let sfx_order = OrderSFX::<AccountId32, u32, u128, [u8; 4], Vec<u8>, u128> {
                sfx_action,
                max_reward: 200u128,
                insurance: 50u128,
                reward_asset: ASSET_DOT,
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

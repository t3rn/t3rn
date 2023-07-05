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
use t3rn_primitives::{circuit::traits::CircuitSubmitAPI, SpeedMode};
pub type Asset = u32;
pub type Destination = [u8; 4];
pub type Input = Vec<u8>;
use t3rn_abi::types::Sfx4bId;
use t3rn_primitives::circuit::SideEffect;
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum SFXAction<Account, Asset, Balance, Destination, Input, MaxCost> {
    // All sorts of calls: composable, wasm, evm, etc. are vacuumed into a single Call SFX in the protocol level.
    Call(Destination, Account, Balance, MaxCost, Input),
    // All of the DEX-related SFXs are vacuumed into a Transfer SFX in the protocol level: swap, add_liquidity, remove_liquidity, transfer asset, transfer native
    Transfer(Asset, Account, Balance),
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct OrderSFX<AccountId, Asset, Balance, Destination, Input, MaxCost> {
    pub sfx_action: SFXAction<AccountId, Asset, Balance, Destination, Input, MaxCost>,
    pub max_reward: Balance,
    pub reward_asset: Asset,
    pub insurance: Balance,
    pub target: Destination,
}

use sp_runtime::DispatchError;

impl<AccountId, Asset, Balance, Destination, Input, MaxCost> TryInto<SideEffect<AccountId, Balance>>
    for OrderSFX<AccountId, Asset, Balance, Destination, Input, MaxCost>
where
    u32: From<Asset>,
    Balance: Encode,
    AccountId: Encode,
{
    type Error = DispatchError;

    fn try_into(self) -> Result<SideEffect<AccountId, Balance>, Self::Error> {
        // Here you would convert the fields of OrderSFX into the fields of SideEffect
        // This is a very basic example and you will need to fill in the details based on your specific requirements

        let _action: Sfx4bId = match self.sfx_action {
            SFXAction::Call(_, _, _, _, _) => *b"call",
            SFXAction::Transfer(_, _, _) => *b"tass",
        };

        let (action, encoded_args) = match self.sfx_action {
            SFXAction::Call(_target, _destination, _value, _max_cost, _input) => {
                let encoded_args = vec![];
                // todo: lookup destination target and derive the ActionType (call evm / wasm / composable)
                // encoded_args.extend_from_slice(&target);
                // encoded_args.extend_from_slice(&destination);
                // encoded_args.extend_from_slice(&value.encode());
                // encoded_args.extend_from_slice(&max_cost.encode());
                // encoded_args.extend_from_slice(&input);
                (*b"call", encoded_args)
            },
            SFXAction::Transfer(asset, destination, amount) => {
                let mut encoded_args: Vec<Vec<u8>> = vec![];

                encoded_args.push(<Asset as Into<u32>>::into(asset).to_le_bytes().to_vec());
                encoded_args.push(destination.encode());
                encoded_args.push(amount.encode());
                (*b"tass", encoded_args)
            },
        };

        // You will need to convert the other fields of OrderSFX into the corresponding fields of SideEffect
        // For example, you might need to encode the arguments for the `encoded_args` field of SideEffect
        // This will depend on the specific details of your implementation

        let side_effect = SideEffect {
            target: Default::default(), // You will need to determine the target based on the details of the SFXAction
            max_reward: self.max_reward,
            insurance: self.insurance,
            action,
            encoded_args, // You will need to encode the arguments based on the details of the SFXAction
            signature: vec![], // You will need to determine the signature based on the details of the SFXAction
            enforce_executor: None, // You will need to determine the executor based on the details of the SFXAction
            reward_asset_id: Some(self.reward_asset.into()), // You will need to determine the asset ID based on the details of the SFXAction
        };

        Ok(side_effect)
    }
}

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
            let _who = ensure_signed(origin.clone())?;

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
    use super::*;
    use frame_support::assert_ok;
    use sp_runtime::AccountId32;

    #[test]
    fn test_try_into_transfer() {
        let order_sfx = OrderSFX::<AccountId32, u32, u128, [u8; 4], Vec<u8>, u128> {
            sfx_action: SFXAction::Transfer(1u32, AccountId32::new([2u8; 32]), 100u128),
            max_reward: 200u128,
            insurance: 50u128,
            reward_asset: 1u32,
            target: [3u8; 4],
        };

        let result: Result<SideEffect<AccountId32, u128>, _> = order_sfx.try_into();
        assert_ok!(&result);

        let side_effect = result.unwrap();
        assert_eq!(side_effect.max_reward, 200);
        assert_eq!(side_effect.insurance, 50);
        assert_eq!(side_effect.action, *b"tass");
        assert_eq!(side_effect.reward_asset_id, Some(1u32));
        assert_eq!(side_effect.encoded_args.len(), 3);
        assert_eq!(side_effect.encoded_args[0], vec![1u8, 0, 0, 0]);
        assert_eq!(side_effect.encoded_args[1], [2u8; 32]);
        assert_eq!(
            side_effect.encoded_args[2],
            vec![100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn test_try_into_call() {
        let order_sfx = OrderSFX::<AccountId32, u32, u128, [u8; 4], Vec<u8>, u128> {
            sfx_action: SFXAction::Call(
                [1u8; 4],
                AccountId32::new([2u8; 32]),
                100u128,
                200u128,
                vec![3u8; 4],
            ),
            max_reward: 200u128,
            insurance: 50u128,
            reward_asset: 1u32,
            target: [3u8; 4],
        };

        let result: Result<SideEffect<AccountId32, u128>, _> = order_sfx.try_into();
        assert_ok!(&result);

        let side_effect = result.unwrap();
        assert_eq!(side_effect.max_reward, 200);
        assert_eq!(side_effect.insurance, 50);
        assert_eq!(side_effect.action, *b"call");
        assert_eq!(side_effect.reward_asset_id, Some(1u32));
    }
}

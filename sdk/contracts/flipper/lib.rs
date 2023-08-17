#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod flipper {
    use t3rn_sdk::{
        error::Error,
        primitives::{
            signal::KillReason,
            state::{ExecutionState, GetSteps},
            xc::{Chain, Operation},
            Box,
        },
    };

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Flipper {
        pub current_step: u32,
    }

    impl Flipper {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { current_step: 0 }
        }

        #[ink(message)]
        pub fn get_current_step(&self) -> u32 {
            self.current_step
        }

        #[ink(message, payable)]
        pub fn t3rn_flip(&mut self, execution_id: Option<Hash>) {
            let caller = self.env().caller().clone();
            let contract = self.env().account_id().clone();

            let _ = t3rn_sdk::execute(
                execution_id,
                Box::new(
                    move |state: &ExecutionState<Hash, AccountId, u64, Balance>| {
                        let mut new_side_effects = t3rn_sdk::Step::default();

                        // we always transfer some dot to the caller for fun
                        new_side_effects.try_push(Chain::<_, _, Hash>::Polkadot(
                            Operation::Transfer {
                                caller,
                                to: contract,
                                amount: 5_u128,
                                insurance: None,
                            },
                        ))?;

                        // if our step is an odd number, lets make a swap on karura
                        if state.get_index() % 2 != 0 {
                            new_side_effects.try_push(Chain::Karura(Operation::Swap {
                                caller,
                                to: contract,
                                amount_from: 1_000_000_u128,
                                amount_to: 50_000_u128,
                                asset_from: Hash::try_from(&b"USD"[..]).unwrap_or_default(),
                                asset_to: Hash::try_from(&b"ETH"[..]).unwrap_or_default(),
                                insurance: None,
                            }))?;
                        }

                        // if our step is 4, lets kill the execution early
                        if state.get_index() == 4 {
                            return Err(Error::ShouldKill(KillReason::Unhandled))
                        }

                        Ok(new_side_effects)
                    },
                ),
            );
        }
    }

    #[cfg(test)]
    mod tests {
        use t3rn_sdk::{
            error::Error,
            primitives::{
                signal::KillReason,
                state::{ExecutionState, GetSteps},
                xc::{Chain, Operation},
                Box,
            },
        };

        #[test]
        fn test() {
            let caller = [1_u8; 32].to_vec();
            let contract = [1_u8; 32].to_vec();
            let mut new_side_effects: t3rn_sdk::Step<Vec<_>, _, u32> = t3rn_sdk::Step::default();

            // we always transfer some dot to the caller for fun
            new_side_effects
                .try_push(Chain::<_, _, _>::Polkadot(Operation::Transfer {
                    caller: caller.clone(),
                    to: contract.clone(),
                    amount: 5_u128,
                    insurance: None,
                }))
                .unwrap();

            // if our step is an odd number, lets make a swap on karura
            new_side_effects
                .try_push(Chain::<_, _, _>::Polkadot(Operation::Transfer {
                    caller,
                    to: contract,
                    amount: 5_u128,
                    insurance: None,
                }))
                .unwrap();
        }
    }
}

use crate::{storage::BoundedVec, Box, Debug, MAX_CALL_LEN};
use codec::{Decode, Encode, MaxEncodedLen};
use t3rn_types::sfx::Insurance;

#[derive(Encode, Decode, MaxEncodedLen, Clone, PartialEq, Eq, Debug)]
pub enum Chain<AccountId, Balance, Hash>
where
    Hash: Encode + Decode,
    AccountId: Encode + Decode,
    Balance: Encode + Decode,
{
    Kusama(Operation<AccountId, Balance, Hash>),
    Polkadot(Operation<AccountId, Balance, Hash>),
    Karura(Operation<AccountId, Balance, Hash>),
    T3rn(Operation<AccountId, Balance, Hash>),
}

impl<AccountId, Balance, Hash> Chain<AccountId, Balance, Hash>
where
    Hash: Encode + Decode,
    AccountId: Encode + Decode,
    Balance: Encode + Decode,
{
    // Could just be deref
    pub fn get_operation(self) -> Operation<AccountId, Balance, Hash> {
        match self {
            Chain::Kusama(op) => op,
            Chain::Polkadot(op) => op,
            Chain::Karura(op) => op,
            Chain::T3rn(op) => op,
        }
    }
}

#[derive(Encode, Decode, MaxEncodedLen, Clone, PartialEq, Eq, Debug)]
pub struct Call<AccountId, Balance>
where
    AccountId: Encode + Decode,
    Balance: Encode + Decode,
{
    pub caller: AccountId,
    pub call: VM<AccountId, Balance>,
    pub data: BoundedVec<u8, MAX_CALL_LEN>,
}

#[derive(Encode, Decode, MaxEncodedLen, Clone, PartialEq, Eq, Debug)]
pub enum Operation<AccountId, Balance, Hash>
where
    Hash: Encode + Decode,
    AccountId: Encode + Decode,
    Balance: Encode + Decode,
{
    Transfer {
        caller: AccountId,
        to: AccountId,
        amount: Balance,
        insurance: Option<Insurance<Balance>>,
    },
    TransferMulti {
        asset: Hash,
        caller: AccountId,
        to: AccountId,
        amount: Balance,
        insurance: Option<Insurance<Balance>>,
    },
    AddLiquidity {
        caller: AccountId,
        to: AccountId,
        asset_left: Hash,
        asset_right: Hash,
        liquidity_token: Hash,
        amount_left: Balance,
        amount_right: Balance,
        amount_liquidity_token: Balance,
        insurance: Option<Insurance<Balance>>,
    },
    Swap {
        caller: AccountId,
        to: AccountId,
        amount_from: Balance,
        amount_to: Balance,
        asset_from: Hash,
        asset_to: Hash,
        insurance: Option<Insurance<Balance>>,
    },
    Call(Box<Call<AccountId, Balance>>),
    Data {
        index: Hash,
    },
}

/// A representation of the type of a call that the VM can handle.
#[derive(Encode, Decode, MaxEncodedLen, Clone, PartialEq, Eq, Debug)]
pub enum VM<AccountId, Balance>
where
    AccountId: Encode + Decode,
    Balance: Encode + Decode,
{
    Evm {
        dest: AccountId,
        value: Balance,
    },
    Wasm {
        dest: AccountId,
        value: Balance,
        gas_limit: Balance,
        storage_limit: Option<Balance>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use scale_info::prelude::vec;

    #[test]
    fn transfer_selector_works() {
        let caller = [5_u8; 2];
        let to = [6_u8; 2];
        let balance = 100_u32;
        let selector = Chain::<_, _, u32>::Kusama(Operation::Transfer {
            caller,
            to,
            amount: balance,
            insurance: None,
        });
        assert_eq!(
            selector.clone().encode(),
            [0, 0, 5, 5, 6, 6, 100, 0, 0, 0, 0]
        );
    }

    #[test]
    fn transfer_multi_selector_works() {
        let caller = [5_u8; 2];
        let to = [6_u8; 2];
        let asset = [7_u8; 2];
        let balance = 100_u32;
        let selector = Chain::Kusama(Operation::TransferMulti {
            caller,
            to,
            amount: balance,
            asset,
            insurance: None,
        });
        assert_eq!(selector.encode(), [0, 1, 7, 7, 5, 5, 6, 6, 100, 0, 0, 0, 0]);
    }

    #[test]
    fn add_liquidity_selector_works() {
        let caller = [5_u8; 2];
        let to = [6_u8; 2];
        let asset_left = [7_u8; 2];
        let asset_right = [8_u8; 2];
        let liquidity_token = [9_u8; 2];
        let amount_left = 100_u32;
        let amount_right = 200_u32;
        let amount_liquidity_token = 300_u32;
        let selector = Chain::Kusama(Operation::AddLiquidity {
            caller,
            to,
            asset_left,
            asset_right,
            liquidity_token,
            amount_left,
            amount_right,
            amount_liquidity_token,
            insurance: None,
        });

        assert_eq!(
            selector.encode(),
            vec![0, 2, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 100, 0, 0, 0, 200, 0, 0, 0, 44, 1, 0, 0, 0]
        );
    }

    #[test]
    fn test_swap_selector_works() {
        let caller = [5_u8; 2];
        let to = [6_u8; 2];
        let asset_from = [7_u8; 2];
        let asset_to = [8_u8; 2];
        let amount_from = 100_u32;
        let amount_to = 200_u32;

        let selector = Chain::Kusama(Operation::Swap {
            caller,
            to,
            amount_from,
            amount_to,
            asset_from,
            asset_to,
            insurance: None,
        });
        assert_eq!(
            selector.encode(),
            vec![0, 3, 5, 5, 6, 6, 100, 0, 0, 0, 200, 0, 0, 0, 7, 7, 8, 8, 0]
        );
    }

    #[test]
    fn test_evm_call_works() {
        let dest = [6_u8; 2];
        let value = 100_u32;
        let call = VM::Evm { dest, value };
        let caller = [5_u8; 2];
        let selector = Chain::<_, u32, u32>::Kusama(Operation::Call(Box::new(Call {
            caller,
            call,
            data: BoundedVec::default(),
        })));
        assert_eq!(
            selector.encode(),
            vec![0, 4, 5, 5, 0, 6, 6, 100, 0, 0, 0, 0]
        );
    }
}

use crate::{AccountEvmAddressMapping, Config, Event, EvmAccountAddressMapping, Pallet};
use codec::Encode;
use frame_support::traits::IsType;
use primitive_types::H160;
use sp_runtime::AccountId32;

pub trait AddressMapping<AccountId> {
    // Returns the AccountId used to generate the given EvmAddress.
    fn get_or_into_account_id(address: &H160) -> AccountId;
    /// Returns the EvmAddress associated with a given AccountId or the
    /// underlying EvmAddress of the AccountId.
    /// Returns None if there is no EvmAddress associated with the AccountId
    /// and there is no underlying EvmAddress in the AccountId.
    fn get_evm_address(account_id: &AccountId) -> Option<H160>;
    /// Returns the EVM address associated with an account ID and generates an
    /// account mapping if no association exists.
    fn get_or_create_evm_address(account_id: &AccountId) -> H160;
}

// Creates a an EvmAddress from an AccountId by appending the bytes "evm:" to
// the account_id and hashing it.
fn account_to_default_evm_address(account_id: &impl Encode) -> H160 {
    let payload = (b"evm:", account_id);
    H160::from_slice(&payload.using_encoded(sp_io::hashing::blake2_256)[0..20])
}

/// Hashed address mapping.
pub struct StoredHashAddressMapping<T>(sp_std::marker::PhantomData<T>);

impl<T: Config> AddressMapping<T::AccountId> for StoredHashAddressMapping<T>
where
    T::AccountId: IsType<AccountId32>,
{
    fn get_or_into_account_id(address: &H160) -> T::AccountId {
        if let Some(acc) = EvmAccountAddressMapping::<T>::get(address) {
            acc
        } else {
            let mut data: [u8; 32] = [0u8; 32];
            data[0..4].copy_from_slice(b"evm:");
            data[4..24].copy_from_slice(&address[..]);
            AccountId32::from(data).into()
        }
    }

    fn get_evm_address(account_id: &T::AccountId) -> Option<H160> {
        // Return the EvmAddress if a mapping to account_id exists
        AccountEvmAddressMapping::<T>::get(account_id).or_else(|| {
            let data: &[u8] = account_id.into_ref().as_ref();
            // Return the underlying EVM address if it exists otherwise return None
            if data.starts_with(b"evm:") {
                Some(H160::from_slice(&data[4..24]))
            } else {
                None
            }
        })
    }

    fn get_or_create_evm_address(account_id: &T::AccountId) -> H160 {
        Self::get_evm_address(account_id).unwrap_or_else(|| {
            let addr = account_to_default_evm_address(account_id);

            // create reverse mapping
            EvmAccountAddressMapping::<T>::insert(addr, account_id);
            AccountEvmAddressMapping::<T>::insert(account_id, addr);

            Pallet::<T>::deposit_event(Event::ClaimAccount {
                account_id: account_id.clone(),
                evm_address: addr,
            });

            addr
        })
    }
}

use crate::{Bytes, DispatchResultWithPostInfo, Error};
use codec::{Decode, Encode};

use sp_core::crypto::ByteArray;
use sp_finality_grandpa::SetId;

use sp_std::vec::Vec;

use t3rn_primitives::bridges::{header_chain as bp_header_chain, runtime as bp_runtime};

pub type CurrentHash<T, I> =
    <<T as pallet_multi_finality_verifier::Config<I>>::BridgedChain as bp_runtime::Chain>::Hash;
pub type CurrentHasher<T, I> =
    <<T as pallet_multi_finality_verifier::Config<I>>::BridgedChain as bp_runtime::Chain>::Hasher;
pub type CurrentHeader<T, I> =
    <<T as pallet_multi_finality_verifier::Config<I>>::BridgedChain as bp_runtime::Chain>::Header;

pub type DefaultPolkadotLikeGateway = ();
pub type PolkadotLikeValU64Gateway = pallet_multi_finality_verifier::Instance1;
pub type EthLikeKeccak256ValU64Gateway = pallet_multi_finality_verifier::Instance2;
pub type EthLikeKeccak256ValU32Gateway = pallet_multi_finality_verifier::Instance3;

pub fn init_bridge_instance<T: pallet_multi_finality_verifier::Config<I>, I: 'static>(
    origin: T::Origin,
    first_header: Vec<u8>,
    authorities: Option<Vec<T::AccountId>>,
    authority_set_id: Option<SetId>,
    gateway_id: bp_runtime::ChainId,
) -> DispatchResultWithPostInfo {
    let header: CurrentHeader<T, I> = Decode::decode(&mut &first_header[..])
        .map_err(|_| "Decoding error: received GenericPrimitivesHeader -> CurrentHeader<T>")?;

    let init_data = bp_header_chain::InitializationData {
        header,
        authority_list: authorities
            .unwrap_or_default()
            .iter()
            .map(|id| sp_finality_grandpa::AuthorityId::from_slice(&id.encode()).unwrap())
            .map(|authority| (authority, 1))
            .collect::<Vec<_>>(),
        set_id: authority_set_id.unwrap_or_default(),
        is_halted: false,
        gateway_id,
    };

    pallet_multi_finality_verifier::Pallet::<T, I>::initialize_single(origin, init_data)
}

pub fn get_roots_from_bridge<T: pallet_multi_finality_verifier::Config<I>, I: 'static>(
    block_hash: Bytes,
    gateway_id: bp_runtime::ChainId,
) -> Result<(sp_core::H256, sp_core::H256), Error<T>> {
    let gateway_block_hash: CurrentHash<T, I> = Decode::decode(&mut &block_hash[..])
        .map_err(|_| Error::<T>::StepConfirmationDecodingError)?;

    let (extrinsics_root, storage_root): (CurrentHash<T, I>, CurrentHash<T, I>) =
        pallet_multi_finality_verifier::Pallet::<T, I>::get_imported_roots(
            gateway_id,
            gateway_block_hash,
        )
        .ok_or(Error::<T>::StepConfirmationBlockUnrecognised)?;

    let extrinsics_root_h256: sp_core::H256 = Decode::decode(&mut &extrinsics_root.encode()[..])
        .map_err(|_| Error::<T>::StepConfirmationDecodingError)?;

    let storage_root_h256: sp_core::H256 = Decode::decode(&mut &storage_root.encode()[..])
        .map_err(|_| Error::<T>::StepConfirmationDecodingError)?;

    Ok((extrinsics_root_h256, storage_root_h256))
}

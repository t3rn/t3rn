use crate::{Bytes, DispatchResultWithPostInfo, Error};
use codec::{Decode, Encode};

use sp_core::crypto::ByteArray;
use sp_finality_grandpa::SetId;
use sp_runtime::traits::Header;

use sp_std::vec::Vec;
use sp_trie::{read_trie_value, LayoutV1, StorageProof};

use t3rn_primitives::{
    bridges::{header_chain as bp_header_chain, runtime as bp_runtime},
    ProofTriePointer,
};

pub type CurrentHash<T, I> =
    <<T as pallet_multi_finality_verifier::Config<I>>::BridgedChain as bp_runtime::Chain>::Hash;
pub type CurrentHasher<T, I> =
    <<T as pallet_multi_finality_verifier::Config<I>>::BridgedChain as bp_runtime::Chain>::Hasher;
pub type CurrentHeader<T, I> =
    <<T as pallet_multi_finality_verifier::Config<I>>::BridgedChain as bp_runtime::Chain>::Header;
pub type CurrentBlockNumber<T, I> =
    <<T as pallet_multi_finality_verifier::Config<I>>::BridgedChain as bp_runtime::Chain>::BlockNumber;

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
) -> Result<(CurrentHash<T, I>, CurrentHash<T, I>), Error<T>> {
    let gateway_block_hash: CurrentHash<T, I> = Decode::decode(&mut &block_hash[..])
        .map_err(|_| Error::<T>::StepConfirmationDecodingError)?;
    let (extrinsics_root, storage_root): (CurrentHash<T, I>, CurrentHash<T, I>) =
        pallet_multi_finality_verifier::Pallet::<T, I>::get_imported_roots(
            gateway_id,
            gateway_block_hash,
        )
        .ok_or(Error::<T>::StepConfirmationBlockUnrecognised)?;

    Ok((extrinsics_root, storage_root))
}

pub fn read_cmp_latest_height_from_bridge<
    T: pallet_multi_finality_verifier::Config<I>,
    I: 'static,
>(
    gateway_id: bp_runtime::ChainId,
    gateway_block_hash: Option<Bytes>,
    maybe_cmp_height: Option<Bytes>,
) -> Result<CurrentBlockNumber<T, I>, Error<T>> {
    let decoded_block_hash: CurrentHash<T, I> = if let Some(encoded_block_hash) = gateway_block_hash
    {
        Decode::decode(&mut &encoded_block_hash[..])
            .map_err(|_| Error::<T>::ReadTargetHeightDecodeBlockHashError)
    } else {
        println!("read_cmp_latest_height_from_bridge before get_bridged_block_hash for {:?}", gateway_id);

        pallet_multi_finality_verifier::Pallet::<T, I>::get_bridged_block_hash(gateway_id)
            .ok_or(Error::<T>::ReadLatestTargetHashError)
    }?;

    println!("read_cmp_latest_height_from_bridge before decoded_block_hash {:?}", decoded_block_hash);

    let header: CurrentHeader<T, I> =
        pallet_multi_finality_verifier::Pallet::<T, I>::get_multi_imported_headers(
            gateway_id,
            decoded_block_hash,
        )
        .ok_or(Error::<T>::ReadTargetHeightDecodeCmpHeightError)?;

    println!("read_cmp_latest_height_from_bridge before header {:?}", header);

    if let Some(encoded_cmp_height) = maybe_cmp_height {
        println!("read_cmp_latest_height_from_bridge encoded_cmp_height {:?}", encoded_cmp_height);
        let cmp_height: CurrentBlockNumber<T, I> = Decode::decode(&mut &encoded_cmp_height[..])
            .map_err(|_e| Error::<T>::ReadTargetHeightDecodeCmpHeightError)?;
        if *header.number() < cmp_height {
            return Err(Error::<T>::ReadTargetHeightReplayAttackDetected.into())
        }
    }

    println!("read_cmp_latest_height_from_bridge return {:?}", *header.number());

    Ok(*header.number())
}

pub fn verify_storage_proof<T: pallet_multi_finality_verifier::Config<I>, I: 'static>(
    block_hash: Bytes,
    gateway_id: bp_runtime::ChainId,
    key: Vec<u8>,
    proof: StorageProof,
    trie_type: ProofTriePointer,
) -> Result<Vec<u8>, Error<T>> {
    return match get_roots_from_bridge::<T, I>(block_hash, gateway_id) {
        Ok((extrinsics_root, storage_root)) => {
            let expected_root = match trie_type {
                ProofTriePointer::State => storage_root,
                ProofTriePointer::Transaction => extrinsics_root,
                ProofTriePointer::Receipts => storage_root,
            };

            let db = proof.into_memory_db::<CurrentHasher<T, I>>();
            let res = read_trie_value::<LayoutV1<CurrentHasher<T, I>>, _>(
                &db,
                &expected_root,
                key.as_ref(),
            );
            match res {
                Ok(Some(value)) => Ok(value),
                _ => Err(Error::<T>::ParachainHeaderNotVerified),
            }
        },
        Err(e) => Err(e),
    }
}

#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
use sp_core::{H160, H256, U256};
use sp_runtime::traits::Block as BlockT;
use sp_std::vec::Vec;

sp_api::decl_runtime_apis! {
    #[api_version(1)]
    pub trait EvmRuntimeRPCApi<AccountId, Balance> where AccountId: Codec, Balance: Codec {
        fn get_evm_address(
            account_id: AccountId,
        ) -> Option<H160>;
        fn get_or_into_account_id(
            address: H160,
        ) -> AccountId;
        fn get_threevm_info(
            address: H160,
        ) -> Option<(AccountId, Balance, u8)>;
        fn account_info(address: H160) -> (U256, U256, Vec<u8>);
        fn storage_at(address: H160, index: U256) -> H256;
    }

    #[api_version(2)]
    pub trait ConvertTransactionRuntimeApi {
        fn convert_transaction(transaction: ethereum::TransactionV2) -> <Block as BlockT>::Extrinsic;
        #[changed_in(2)]
        fn convert_transaction(transaction: ethereum::TransactionV0) -> <Block as BlockT>::Extrinsic;
    }
}

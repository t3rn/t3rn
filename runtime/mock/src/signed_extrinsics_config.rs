use super::*;

/// Block type as expected by this runtime.
pub type Block = sp_runtime::generic::Block<Header, UncheckedExtrinsic>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
    frame_system::CheckNonZeroSender<Runtime>,
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckEra<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
    pallet_asset_tx_payment::ChargeAssetTxPayment<Runtime>,
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
    sp_runtime::generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic =
    sp_runtime::generic::CheckedExtrinsic<AccountId, RuntimeCall, SignedExtra>;

// impl<C> frame_system::offchain::SendTransactionTypes<C> for Runtime
// where
//     RuntimeCall: From<C>,
// {
//     type Extrinsic = Extrinsic;
//     type OverarchingCall = RuntimeCall;
// }

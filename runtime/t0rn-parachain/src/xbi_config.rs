use crate::*;

use frame_support::{parameter_types, traits::ConstU32};
use sp_core::H256;

use frame_support::dispatch::{DispatchError, DispatchErrorWithPostInfo};

use pallet_xbi_portal::{
    primitives::xbi::XBIStatus,
    xbi_codec::XBIFormat,
    xbi_format::{XBICheckIn, XBICheckOut},
    Error,
};

impl<C> frame_system::offchain::SendTransactionTypes<C> for Runtime
where
    Call: From<C>,
{
    // type Extrinsic = TestXt<Call, ()>;
    type Extrinsic = UncheckedExtrinsic;
    type OverarchingCall = Call;
}

parameter_types! {
    pub const SelfGatewayId: [u8; 4] = [3, 3, 3, 3];
    pub const XbiSovereign: AccountId = AccountId::new([68u8; 32]); // 0x444...4
}

// ToDo: Implement
impl codec::EncodeLike<pallet_xbi_portal::Call<Runtime>> for Call {}

impl pallet_xbi_portal::Config for Runtime {
    type AssetRegistry = AssetRegistry;
    type Assets = Assets;
    type Call = Call;
    // type SelfAccountId = XbiSovereign;
    // type Callback = XBIPortalRuntimeEntry;
    type Callback = ();
    type CheckInLimit = ConstU32<100>;
    type CheckInterval = ConstU64<3>;
    type CheckOutLimit = ConstU32<100>;
    type Contracts = Contracts;
    type Currency = Balances;
    type DeFi = ();
    type Event = Event;
    type Evm = Evm;
    type ExpectedBlockTimeMs = ConstU32<6000>;
    type ParachainId = ConstU32<3333>;
    type TimeoutChecksLimit = ConstU32<3000>;
    type Xcm = XcmRouter;
    type XcmSovereignOrigin = XbiSovereign;
}

// pub struct XBIPortalRuntimeEntry {}

// impl pallet_xbi_portal::primitives::xbi::XBIPortal<Runtime> for XBIPortalRuntimeEntry {
//     fn do_check_in_xbi(xbi: XBIFormat) -> Result<(), Error<Runtime>> {
//         XBIPortal::do_check_in_xbi(xbi)
//     }

//     fn get_status(xbi_id: H256) -> XBIStatus {
//         XBIPortal::get_status(xbi_id)
//     }

//     fn get_check_in(
//         xbi_id: H256,
//     ) -> Result<XBICheckIn<<Runtime as frame_system::Config>::BlockNumber>, DispatchError> {
//         XBIPortal::get_check_in(xbi_id)
//     }

//     fn get_check_out(xbi_id: H256) -> Result<XBICheckOut, DispatchError> {
//         XBIPortal::get_check_out(xbi_id)
//     }
// }

// impl pallet_xbi_portal::primitives::xbi_callback::XBICallback<Runtime> for XBIPortalRuntimeEntry {
//     fn callback(xbi_checkin: XBICheckIn<BlockNumber>, xbi_checkout: XBICheckOut) {
//         Circuit::do_xbi_exit(xbi_checkin, xbi_checkout);
//     }
// }

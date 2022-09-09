use crate::*;

use frame_support::{parameter_types, traits::ConstU32, PalletId};
use sp_core::H256;
use sp_runtime::traits::*;

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
    pub const CircuitPalletId: PalletId = PalletId(*b"pal/circ");
    pub const XBIPalletId: PalletId = PalletId(*b"pal/xbip");
    pub const SelfGatewayId: [u8; 4] = [3, 3, 3, 3];
}

// ToDo: Implement
impl codec::EncodeLike<pallet_xbi_portal::Call<Runtime>> for Call {}

impl pallet_xbi_portal::Config for Runtime {
    type Assets = pallet_xbi_portal::primitives::assets::AssetsMock<Runtime>;
    type Call = Call;
    type Callback = XBIPortalRuntimeEntry;
    type CheckInLimit = ConstU32<100>;
    type CheckInterval = ConstU32<3>;
    type CheckOutLimit = ConstU32<100>;
    type Event = Event;
    type Evm = pallet_xbi_portal::primitives::evm::EvmMock<Runtime>;
    type ExpectedBlockTimeMs = ConstU32<6000>;
    type MyParachainId = ConstU32<3333>;
    type ORML = pallet_xbi_portal::primitives::orml::ORMLMock<Runtime>;
    type PalletId = XBIPalletId;
    type TimeoutChecksLimit = ConstU32<3000>;
    type Transfers = XBIPortalRuntimeEntry;
    type WASM = pallet_xbi_portal::primitives::wasm::WASMMock<Runtime>;
    type Xcm = pallet_xbi_portal::primitives::xcm::XCMMock<Runtime>;
}

pub struct XBIPortalRuntimeEntry {}

impl pallet_xbi_portal::primitives::xbi::XBIPortal<Runtime> for XBIPortalRuntimeEntry {
    fn do_check_in_xbi(xbi: XBIFormat) -> Result<(), Error<Runtime>> {
        XBIPortal::do_check_in_xbi(xbi)
    }

    fn get_status(xbi_id: H256) -> XBIStatus {
        XBIPortal::get_status(xbi_id)
    }

    fn get_check_in(
        xbi_id: H256,
    ) -> Result<XBICheckIn<<Runtime as frame_system::Config>::BlockNumber>, DispatchError> {
        XBIPortal::get_check_in(xbi_id)
    }

    fn get_check_out(xbi_id: H256) -> Result<XBICheckOut, DispatchError> {
        XBIPortal::get_check_out(xbi_id)
    }
}

impl pallet_xbi_portal::primitives::xbi_callback::XBICallback<Runtime> for XBIPortalRuntimeEntry {
    fn callback(xbi_checkin: XBICheckIn<BlockNumber>, xbi_checkout: XBICheckOut) {
        Circuit::do_xbi_exit(xbi_checkin, xbi_checkout);
    }
}

impl pallet_xbi_portal::primitives::transfers::Transfers<Runtime> for XBIPortalRuntimeEntry {
    fn transfer(
        source: &AccountId,
        dest: &AccountId,
        amount: Balance,
        _keep_alive: bool,
    ) -> Result<frame_support::dispatch::PostDispatchInfo, DispatchErrorWithPostInfo> {
        Balances::transfer(
            Origin::signed(source.clone()),
            sp_runtime::MultiAddress::Id(dest.clone()),
            amount,
        )
    }
}

impl pallet_xbi_portal_enter::Config for Runtime {
    type Event = Event;
    type XBIPortal = XBIPortal;
}

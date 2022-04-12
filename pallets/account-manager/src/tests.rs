use super::*;
use crate::mock::{ExtBuilder, Test, XDNS};
use codec::Decode;
use frame_support::{assert_err, assert_noop, assert_ok};
use frame_system::Origin;
use sp_runtime::DispatchError;
use t3rn_primitives::{abi::Type, xdns::Xdns, GatewayType, GatewayVendor};

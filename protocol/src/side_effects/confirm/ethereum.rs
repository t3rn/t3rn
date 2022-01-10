#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use ethabi_decode::{Event, Param, ParamKind, Token};
use snowbridge_core::{Message, Verifier};
use snowbridge_ethereum::{Header, Log};
use sp_core::{H160, U256};

use sp_runtime::{DispatchError, RuntimeDebug};
use sp_std::convert::TryFrom;
use sp_std::default::Default;
use sp_std::marker::PhantomData;
use sp_std::vec;
use sp_std::vec::Vec;

use crate::side_effects::confirm::parser::VendorSideEffectsParser;

type Bytes = Vec<u8>;
type Arguments = Vec<Bytes>;
pub type EventSignature = Vec<u8>;
pub type String = Vec<u8>;

pub struct EthereumSideEffectsParser<Verifier>(PhantomData<Verifier>);

impl<V: Verifier> VendorSideEffectsParser for EthereumSideEffectsParser<V> {
    fn parse_event<T: pallet_balances::Config>(
        name: &'static str,
        encoded_data: Vec<u8>,
        _signature: &'static str,
    ) -> Result<Arguments, &'static str> {
        match name {
            "transfer:dirty" => {
                let msg: Message = Decode::decode(&mut encoded_data.as_slice())
                    .map_err(|_| "failed to decode eth message")?;
                let log = V::verify(&msg).map_err(|_| "failed to verify eth message")?;
                // Decode log into an Envelope
                let transfer =
                    TransferERC20::try_from(log).map_err(|_| "failed to decode transfer event")?;
                Ok(transfer.to_args())
            }
            &_ => Err("unknown eth event"),
        }
    }
}

// Used to decode a raw Ethereum log into an [`Envelope`].
static EVENT_ABI: &Event = &Event {
    signature: "Transfer(address indexed,address indexed,uint256)",
    inputs: &[
        Param {
            kind: ParamKind::Address,
            indexed: true,
        },
        Param {
            kind: ParamKind::Address,
            indexed: true,
        },
        Param {
            kind: ParamKind::Uint(256),
            indexed: false,
        },
    ],
    anonymous: false,
};

/// An inbound message that has had its outer envelope decoded.
#[derive(Clone, PartialEq, Eq, RuntimeDebug)]
pub struct TransferERC20 {
    pub from: H160,
    pub to: H160,
    pub amount: U256,
}

#[derive(Copy, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct EnvelopeDecodeError;

pub struct EthereumMockVerifier {}

// TODO: Implement proper Ethereum Verifier
impl Verifier for EthereumMockVerifier {
    fn verify(_message: &Message) -> Result<Log, DispatchError> {
        Ok(Default::default())
    }

    fn initialize_storage(
        _headers: Vec<Header>,
        _initial_difficulty: U256,
        _descendants_until_final: u8,
    ) -> Result<(), &'static str> {
        Ok(())
    }
}

impl TryFrom<Log> for TransferERC20 {
    type Error = EnvelopeDecodeError;

    fn try_from(log: Log) -> Result<Self, Self::Error> {
        let tokens = EVENT_ABI
            .decode(log.topics, log.data)
            .map_err(|_| EnvelopeDecodeError)?;

        let mut iter = tokens.into_iter();

        let from = match iter.next().ok_or(EnvelopeDecodeError)? {
            Token::Address(addr) => addr,
            _ => return Err(EnvelopeDecodeError),
        };

        let to = match iter.next().ok_or(EnvelopeDecodeError)? {
            Token::Address(addr) => addr,
            _ => return Err(EnvelopeDecodeError),
        };

        let amount = match iter.next().ok_or(EnvelopeDecodeError)? {
            Token::Uint(amount) => amount,
            _ => return Err(EnvelopeDecodeError),
        };

        Ok(Self { from, to, amount })
    }
}

impl TransferERC20 {
    fn to_args(&self) -> Arguments {
        let mut args = vec![];
        // scale encoded args
        args.push(self.from.encode());
        args.push(self.to.encode());
        args.push(self.amount.encode());
        args
    }
}

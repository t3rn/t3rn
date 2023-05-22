use frame_support::pallet_prelude::*;
use sp_application_crypto::{ecdsa, ed25519, sr25519, KeyTypeId, RuntimePublic};
use sp_core::{crypto::Ss58Codec, sr25519::Public, H160, H256};
use sp_runtime::Percent;
use sp_std::{convert::TryInto, prelude::*};
use std::io::Read;
use t3rn_types::sfx::TargetId;

// Key types for attester crypto
pub const ECDSA_ATTESTER_KEY_TYPE_ID: KeyTypeId = KeyTypeId(*b"ecat");
pub const ED25519_ATTESTER_KEY_TYPE_ID: KeyTypeId = KeyTypeId(*b"edat");
pub const SR25519_ATTESTER_KEY_TYPE_ID: KeyTypeId = KeyTypeId(*b"srat");

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
pub struct AttesterInfo {
    pub key_ed: [u8; 32],
    pub key_ec: [u8; 33],
    pub key_sr: [u8; 32],
    pub commission: Percent,
    pub eth_address: H160,
    pub substrate_address: Vec<u8>,
    pub index: u32,
}

use secp256k1::PublicKey;

use crate::ExecutionVendor;
use tiny_keccak::{Hasher, Keccak};

pub fn ecdsa_pubkey_to_eth_address(pubkey: &PublicKey) -> [u8; 20] {
    let serialized = pubkey.serialize_uncompressed();

    // Remove the first byte (0x04) from the 65-byte serialized public key.
    // Ethereum addresses represent the Keccak-256 hash of the public key (sans the 0x04 byte),
    // rightmost 20 bytes
    let without_prefix = &serialized[1..];

    // Hash with Keccak-256
    let mut hasher = Keccak::v256();
    hasher.update(without_prefix);
    let mut output = [0u8; 32];
    hasher.finalize(&mut output);

    // Take the last 20 bytes
    let mut address = [0u8; 20];
    address.copy_from_slice(&output[12..]);
    address
}

#[test]
fn test_ecdsa_pubkey_to_eth_address() {
    let public_key = PublicKey::from_slice(&[
        0x02, 0xc6, 0x6e, 0x7d, 0x89, 0x66, 0xb5, 0xc5, 0x55, 0xaf, 0x58, 0x05, 0x98, 0x9d, 0xa9,
        0xfb, 0xf8, 0xdb, 0x95, 0xe1, 0x56, 0x31, 0xce, 0x35, 0x8c, 0x3a, 0x17, 0x10, 0xc9, 0x62,
        0x67, 0x90, 0x63,
    ])
    .unwrap();
    let address = ecdsa_pubkey_to_eth_address(&public_key);
    assert_eq!(
        hex::encode(address),
        "efc8e898b7a8376ea9e6feeebfe2a67aebe923f6"
    );
}

impl AttesterInfo {
    pub fn verify_attestation_signature_against_address(
        &self,
        key_type: KeyTypeId,
        message: &Vec<u8>,
        signature: &[u8],
        target_execution_vendor: &ExecutionVendor,
    ) -> Result<bool, DispatchError> {
        self.verify_attestation_signature(key_type, message, signature)?;

        match target_execution_vendor {
            ExecutionVendor::EVM => {
                let pubkey_as_secp = PublicKey::from_slice(&self.key_ec.as_slice())
                    .map_err(|_| "InvalidPublicKey")?;
                let recovered_address = ecdsa_pubkey_to_eth_address(&pubkey_as_secp);
                Ok(H160(recovered_address) == self.eth_address)
            },
            ExecutionVendor::Substrate => {
                let pubkey_as_sr = sr25519::Public::from_raw(self.key_sr);
                let mut address_string_buffer = String::new();
                let mut address_slice = self.substrate_address.as_slice();
                address_slice
                    .read_to_string(&mut address_string_buffer)
                    .map_err(|_| "InvalidAddress")?;

                let address_as_sr = Public::from_ss58check(address_string_buffer.as_str())
                    .map_err(|_| "InvalidAddress")?;

                Ok(address_as_sr == pubkey_as_sr)
            },
        }
    }

    pub fn verify_attestation_signature(
        &self,
        key_type: KeyTypeId,
        message: &Vec<u8>,
        signature: &[u8],
    ) -> Result<bool, DispatchError> {
        match key_type {
            ECDSA_ATTESTER_KEY_TYPE_ID => {
                let ecdsa_sig = ecdsa::Signature::from_slice(signature)
                    .ok_or::<DispatchError>("InvalidSignature".into())?;
                let ecdsa_public = ecdsa::Public::from_raw(self.key_ec);
                Ok(ecdsa_public.verify(message, &ecdsa_sig))
            },
            ED25519_ATTESTER_KEY_TYPE_ID => {
                let ed25519_sig = ed25519::Signature::from_slice(signature)
                    .ok_or::<DispatchError>("InvalidSignature".into())?;
                let ed25519_public = ed25519::Public::from_raw(self.key_ed);
                Ok(ed25519_public.verify(message, &ed25519_sig))
            },
            SR25519_ATTESTER_KEY_TYPE_ID => {
                let sr25519_sig = sr25519::Signature::from_slice(signature)
                    .ok_or::<DispatchError>("InvalidSignature".into())?;
                let sr25519_public = sr25519::Public::from_raw(self.key_sr);
                Ok(sr25519_public.verify(message, &sr25519_sig))
            },
            _ => Err("InvalidKeyTypeId".into()),
        }
    }
}

pub type Signature65b = [u8; 65];
pub type PublicKeyEcdsa33b = [u8; 33];
pub const COMMITTEE_SIZE: usize = 32;

pub type CommitteeTransition = [u32; COMMITTEE_SIZE];
pub type AttestersChange = Vec<([u8; 33], u32)>;
pub type BatchConfirmedSfxId = Vec<H256>;

pub trait AttestersWriteApi<Account, Error> {
    fn request_sfx_attestation(target: TargetId, sfx_id: H256) -> Result<(), Error>;
    fn request_add_attesters_attestation(add_attester: &Account) -> Result<(), Error>;
    fn request_ban_attesters_attestation(ban_attesters: &Account) -> Result<(), Error>;
    fn request_remove_attesters_attestation(remove_attesters: &Account) -> Result<(), Error>;
    fn request_next_committee_attestation(next_committee: CommitteeTransition);
}

pub trait AttestersReadApi<Account, Balance> {
    fn previous_committee() -> Vec<Account>;
    fn current_committee() -> Vec<Account>;
    fn active_set() -> Vec<Account>;
    fn honest_active_set() -> Vec<Account>;
    fn read_attester_info(attester: &Account) -> Option<AttesterInfo>;
    fn read_nominations(for_attester: &Account) -> Vec<(Account, Balance)>;
}

pub struct AttestersReadApiEmptyMock<Account, Balance> {
    _phantom: PhantomData<(Account, Balance)>,
}

impl<Account, Balance> AttestersReadApi<Account, Balance>
    for AttestersReadApiEmptyMock<Account, Balance>
{
    fn previous_committee() -> Vec<Account> {
        vec![]
    }

    fn current_committee() -> Vec<Account> {
        vec![]
    }

    fn active_set() -> Vec<Account> {
        vec![]
    }

    fn honest_active_set() -> Vec<Account> {
        vec![]
    }

    fn read_attester_info(_attester: &Account) -> Option<AttesterInfo> {
        None
    }

    fn read_nominations(_for_attester: &Account) -> Vec<(Account, Balance)> {
        vec![]
    }
}

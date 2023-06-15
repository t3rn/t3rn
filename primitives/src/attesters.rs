use crate::GatewayVendor;

use frame_support::pallet_prelude::*;
use sp_application_crypto::{ecdsa, ed25519, sr25519, KeyTypeId, RuntimePublic};
use sp_core::{H160, H256};
use sp_runtime::Percent;
use sp_std::prelude::*;
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
    pub index: u32,
}

use tiny_keccak::{Hasher, Keccak};

pub fn ecdsa_pubkey_to_eth_address(pubkey: &[u8; 33]) -> Result<[u8; 20], DispatchError> {
    let pubkey = libsecp256k1::PublicKey::parse_slice(
        pubkey.as_slice(),
        Some(libsecp256k1::PublicKeyFormat::Compressed),
    )
    .map_err(|_| {
        DispatchError::Other(
            "Failed to parse ECDSA public key - Compressed 33b secp256k1 PK expected",
        )
    })?;

    let serialized = pubkey.serialize();

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
    Ok(address)
}

#[test]
fn test_ecdsa_pubkey_to_eth_address() {
    let _pk_hex = hex_literal::hex!("79846fd12ed97f908d879fc03f1893eb1a18fb3e76d431d31602dd50f50fd9eff78e4603b994db0873fccc41e6ee7846e8050ea4909b4c5d89daf5a40b58b762");

    let compressed_ecdsa_pub_key: [u8; 33] = [
        3, 213, 51, 13, 232, 85, 194, 30, 34, 218, 22, 60, 149, 40, 220, 34, 77, 173, 31, 61, 164,
        213, 17, 67, 159, 112, 25, 151, 30, 247, 76, 130, 145,
    ];
    let address_res = ecdsa_pubkey_to_eth_address(&compressed_ecdsa_pub_key);

    frame_support::assert_ok!(address_res);
    let address = address_res.unwrap();

    assert_eq!(
        hex::encode(address),
        "1e8f2abdffa8bf75802d24b5329d2351b6ab3486"
    );
}

impl AttesterInfo {
    pub fn verify_attestation_signature(
        &self,
        key_type: KeyTypeId,
        message: &Vec<u8>,
        signature: &[u8],
        attested_recoverable: Vec<u8>,
        target_finality: &GatewayVendor,
    ) -> Result<bool, DispatchError> {
        match key_type {
            ECDSA_ATTESTER_KEY_TYPE_ID => {
                let ecdsa_sig = ecdsa::Signature::from_slice(signature)
                    .ok_or::<DispatchError>("InvalidSignature".into())?;
                let ecdsa_public = ecdsa::Public::from_raw(self.key_ec);
                if target_finality == &GatewayVendor::Ethereum {
                    let recovered_address = ecdsa_pubkey_to_eth_address(&self.key_ec)?;
                    let attested_recoverable = H160::decode(&mut &attested_recoverable[..])
                        .map_err(|_| "InvalidRecoverable")?;

                    if H160(recovered_address) != attested_recoverable {
                        return Ok(false)
                    }
                }
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

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo, Default)]
pub enum LatencyStatus {
    #[default]
    OnTime,
    // Late: (n amount of missed latency windows, total amount of successful repatriations)
    Late(u32, u32),
}

pub type CommitteeTransitionIndices = [u32; COMMITTEE_SIZE];
#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo, Default)]
pub struct GenericCommitteeTransition([(u32, Vec<u8>); COMMITTEE_SIZE]);

pub type EvmCommitteeTransition = [(u32, H160); COMMITTEE_SIZE];
pub type CommitteeTransition = Vec<(u32, Vec<u8>)>;
pub type CommitteeRecoverable = Vec<Vec<u8>>;
pub type CommitteeTransitionEncoded = Vec<u8>;

pub type AttestersChange = Vec<([u8; 33], u32)>;
pub type BatchConfirmedSfxId = Vec<H256>;

pub trait AttestersWriteApi<Account, Error> {
    fn request_sfx_attestation_commit(target: TargetId, sfx_id: H256) -> Result<(), Error>;
    fn request_sfx_attestation_revert(target: TargetId, sfx_id: H256) -> Result<(), Error>;
    fn request_ban_attesters_attestation(ban_attesters: &Account) -> Result<(), Error>;
    fn request_next_committee_attestation();
}

pub trait AttestersReadApi<Account, Balance> {
    fn previous_committee() -> Vec<Account>;
    fn current_committee() -> Vec<Account>;
    fn active_set() -> Vec<Account>;
    fn honest_active_set() -> Vec<Account>;
    fn read_attester_info(attester: &Account) -> Option<AttesterInfo>;
    fn read_nominations(for_attester: &Account) -> Vec<(Account, Balance)>;
    fn get_activated_targets() -> Vec<TargetId>;
    fn read_attestation_latency(target: &TargetId) -> Option<LatencyStatus>;
}

pub struct AttestersReadApiEmptyMock<Account, Balance, Error> {
    _phantom: PhantomData<(Account, Balance, Error)>,
}

impl<Account, Balance, Error> AttestersReadApi<Account, Balance>
    for AttestersReadApiEmptyMock<Account, Balance, Error>
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

    fn get_activated_targets() -> Vec<TargetId> {
        vec![]
    }

    fn read_attestation_latency(target: &TargetId) -> Option<LatencyStatus> {
        None
    }
}

impl<Account, Balance, Error> AttestersWriteApi<Account, Error>
    for AttestersReadApiEmptyMock<Account, Balance, Error>
{
    fn request_sfx_attestation_commit(_target: TargetId, _sfx_id: H256) -> Result<(), Error> {
        Ok(())
    }

    fn request_sfx_attestation_revert(_target: TargetId, _sfx_id: H256) -> Result<(), Error> {
        Ok(())
    }

    fn request_ban_attesters_attestation(_ban_attesters: &Account) -> Result<(), Error> {
        Ok(())
    }

    fn request_next_committee_attestation() {}
}

#[cfg(test)]
pub mod test {
    use super::{AttestersReadApi, AttestersReadApiEmptyMock, AttestersWriteApi};
    use frame_support::assert_ok;
    use sp_core::crypto::AccountId32;
    use sp_runtime::DispatchError;
    use t3rn_types::fsx::TargetId;

    #[test]
    fn attesters_mocks_return_empty_data() {
        let attester_rw_mock: AttestersReadApiEmptyMock<AccountId32, u128, DispatchError> =
            AttestersReadApiEmptyMock {
                _phantom: Default::default(),
            };

        assert_ok!(
            <AttestersReadApiEmptyMock<AccountId32, u128, DispatchError> as AttestersWriteApi<
                AccountId32,
                DispatchError,
            >>::request_sfx_attestation_commit([0u8; 4], sp_core::H256([0u8; 32]))
        );

        assert_ok!(
            <AttestersReadApiEmptyMock<AccountId32, u128, DispatchError> as AttestersWriteApi<
                AccountId32,
                DispatchError,
            >>::request_sfx_attestation_revert([0u8; 4], sp_core::H256([0u8; 32]))
        );

        assert_ok!(
            <AttestersReadApiEmptyMock<AccountId32, u128, DispatchError> as AttestersWriteApi<
                AccountId32,
                DispatchError,
            >>::request_ban_attesters_attestation(&AccountId32::new([0; 32]))
        );

        assert_eq!(
            <AttestersReadApiEmptyMock<AccountId32, u128, DispatchError> as AttestersWriteApi<
                AccountId32,
                DispatchError,
            >>::request_next_committee_attestation(),
            ()
        );

        assert_eq!(
            <AttestersReadApiEmptyMock<AccountId32, u128, DispatchError> as AttestersReadApi<
                AccountId32,
                u128,
            >>::previous_committee(),
            vec![]
        );

        assert_eq!(
            <AttestersReadApiEmptyMock<AccountId32, u128, DispatchError> as AttestersReadApi<
                AccountId32,
                u128,
            >>::current_committee(),
            vec![]
        );

        assert_eq!(
            <AttestersReadApiEmptyMock<AccountId32, u128, DispatchError> as AttestersReadApi<
                AccountId32,
                u128,
            >>::active_set(),
            vec![]
        );

        assert_eq!(
            <AttestersReadApiEmptyMock<AccountId32, u128, DispatchError> as AttestersReadApi<
                AccountId32,
                u128,
            >>::honest_active_set(),
            vec![]
        );

        assert_eq!(
            <AttestersReadApiEmptyMock<AccountId32, u128, DispatchError> as AttestersReadApi<
                AccountId32,
                u128,
            >>::read_attester_info(&AccountId32::new([0; 32])),
            None
        );

        assert_eq!(
            <AttestersReadApiEmptyMock<AccountId32, u128, DispatchError> as AttestersReadApi<
                AccountId32,
                u128,
            >>::read_nominations(&AccountId32::new([0; 32])),
            vec![]
        );

        assert_eq!(
            <AttestersReadApiEmptyMock<AccountId32, u128, DispatchError> as AttestersReadApi<
                AccountId32,
                u128,
            >>::get_activated_targets(),
            Vec::<TargetId>::new()
        );
    }
}

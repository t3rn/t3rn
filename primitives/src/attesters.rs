use crate::GatewayVendor;

use frame_support::pallet_prelude::*;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_application_crypto::{ecdsa, ed25519, sr25519, KeyTypeId, RuntimePublic};
use sp_core::{H160, H256};
use sp_runtime::{traits::Zero, Percent};
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

#[test]
fn test_ecdsa_verify_attestation_signature_derives_expected_eth_address_for_ethers_sign_message() {
    use hex_literal::hex;

    // test for example eth keys setting:
    //     "ethereum": {
    //       "privateKey": "0x115db6b0c74bef87e28879199e3ab3dda09ed0e7f0c3e1ff6cb92e228b221384",
    //       "publicKey": "0x026c443c26ef9634344358a4848297ea45d09b59922aa4216c6e6ac97a7de37473",
    //       "publicKeyUncompressed": "0x046c443c26ef9634344358a4848297ea45d09b59922aa4216c6e6ac97a7de3747361fedf02da3d46d5a859ab9b306561fcaefa9d486ae3eef1de7344e3252ad0be",
    //       "address": "0x3a68c6b6f010017c9b330a7c86d4b19c46ab677a"
    //     },

    let compressed_ecdsa_pub_key: [u8; 33] =
        hex!("026c443c26ef9634344358a4848297ea45d09b59922aa4216c6e6ac97a7de37473");

    let message: [u8; 32] =
        hex!("58cd0ea9f78f115b381b29bc7edaab46f214968c05ff24b6b14474e4e47cfcdd");

    let address_res = ecdsa_pubkey_to_eth_address(&compressed_ecdsa_pub_key);

    frame_support::assert_ok!(address_res);
    let address = address_res.unwrap();

    assert_eq!(
        hex::encode(address),
        "3a68c6b6f010017c9b330a7c86d4b19c46ab677a"
    );

    let attester_info = AttesterInfo {
        key_ed: [0u8; 32],
        key_ec: compressed_ecdsa_pub_key,
        key_sr: [0u8; 32],
        commission: Percent::from_percent(0),
        index: 0,
    };

    // Expected value from contracts tests: AttestationSignature::Should recover the correct signer from the signature ethers sign message
    let signature: [u8; 65] = hex!("3c20151678cbbf6c3547c5f911c613e630b0e1be11b24b6b815582db0e47801175421540c660de2a93b46e48f9ff503e5858279ba157fa9b13fbee0a8cf6806e1c");

    let verify_result = attester_info.verify_attestation_signature(
        ECDSA_ATTESTER_KEY_TYPE_ID,
        &message.to_vec(),
        signature.as_ref(),
        address.to_vec(),
        &GatewayVendor::Ethereum,
    );

    frame_support::assert_ok!(verify_result);
    assert_eq!(verify_result, Ok(true));
}

#[test]
fn test_ecdsa_verify_attestation_signature_derives_expected_eth_address_for_eth_utils_ecsign() {
    use hex_literal::hex;

    // test for example eth keys setting:
    //     "ethereum": {
    //       "privateKey": "0x115db6b0c74bef87e28879199e3ab3dda09ed0e7f0c3e1ff6cb92e228b221384",
    //       "publicKey": "0x026c443c26ef9634344358a4848297ea45d09b59922aa4216c6e6ac97a7de37473",
    //       "publicKeyUncompressed": "0x046c443c26ef9634344358a4848297ea45d09b59922aa4216c6e6ac97a7de3747361fedf02da3d46d5a859ab9b306561fcaefa9d486ae3eef1de7344e3252ad0be",
    //       "address": "0x3a68c6b6f010017c9b330a7c86d4b19c46ab677a"
    //     },

    let compressed_ecdsa_pub_key: [u8; 33] =
        hex!("026c443c26ef9634344358a4848297ea45d09b59922aa4216c6e6ac97a7de37473");

    let message: [u8; 32] =
        hex!("58cd0ea9f78f115b381b29bc7edaab46f214968c05ff24b6b14474e4e47cfcdd");

    let address_res = ecdsa_pubkey_to_eth_address(&compressed_ecdsa_pub_key);

    frame_support::assert_ok!(address_res);
    let address = address_res.unwrap();

    assert_eq!(
        hex::encode(address),
        "3a68c6b6f010017c9b330a7c86d4b19c46ab677a"
    );

    let attester_info = AttesterInfo {
        key_ed: [0u8; 32],
        key_ec: compressed_ecdsa_pub_key,
        key_sr: [0u8; 32],
        commission: Percent::from_percent(0),
        index: 0,
    };

    // Expected value from contracts tests: AttestationSignature::Should recover the correct signer from the signature escsign
    let signature: [u8; 65] = hex!("97748ab697916ad7992e8d000360b1a44c8faf6d98b70632a1ce826ff50e995e4335f3234bd6964a722ca7ef95b731568d53499e62b078346fcb5790c94833171b");

    let verify_result = attester_info.verify_attestation_signature(
        ECDSA_ATTESTER_KEY_TYPE_ID,
        &message.to_vec(),
        signature.as_ref(),
        address.to_vec(),
        &GatewayVendor::Ethereum,
    );

    frame_support::assert_ok!(verify_result);
    assert_eq!(verify_result, Ok(true));
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
                    return Ok(recovered_address.to_vec() == attested_recoverable)
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
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum LatencyStatus {
    #[default]
    OnTime,
    /// Late: (n amount of missed latency windows, total amount of successful repatriations)
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

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo, Default)]
pub struct BatchingFactor {
    pub latest_confirmed: u16,
    pub latest_signed: u16,
    pub current_next: u16,
    pub up_to_last_10_confirmed: Vec<u16>,
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
    // Estimate finality fee for user including set overcharge factor (32%)
    fn estimate_finality_fee(target: &TargetId) -> Balance;
    // Estimate finality reward for executor based on the current estimated batching factor
    fn estimate_finality_reward(target: &TargetId) -> Balance;
    fn estimate_batching_factor(target: &TargetId) -> Option<BatchingFactor>;
    // fn estimate_future_user_base(batching_factor: &BatchingFactor, n_epochs_ahead: u16) -> u16;
}

pub struct AttestersReadApiEmptyMock<Account, Balance, Error> {
    _phantom: PhantomData<(Account, Balance, Error)>,
}

impl<Account, Balance: Zero, Error> AttestersReadApi<Account, Balance>
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

    fn read_attestation_latency(_target: &TargetId) -> Option<LatencyStatus> {
        None
    }

    fn estimate_finality_fee(_target: &TargetId) -> Balance {
        // Balance::from(0)
        Zero::zero()
    }

    fn estimate_finality_reward(_target: &TargetId) -> Balance {
        // Balance::from(0)
        Zero::zero()
    }

    fn estimate_batching_factor(_target: &TargetId) -> Option<BatchingFactor> {
        Some(BatchingFactor {
            latest_confirmed: 0,
            latest_signed: 0,
            current_next: 0,
            up_to_last_10_confirmed: vec![],
        })
    }

    // fn estimate_future_user_base(batching_factor: &BatchingFactor, n_epochs_ahead: u16) -> u16 {
    //     // 0
    //     Zero::zero()
    // }
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
        let _attester_rw_mock: AttestersReadApiEmptyMock<AccountId32, u128, DispatchError> =
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

        assert_eq!(
            <AttestersReadApiEmptyMock<AccountId32, u128, DispatchError> as AttestersReadApi<
                AccountId32,
                u128,
            >>::read_attestation_latency(&[0u8; 4]),
            None
        );
    }
}

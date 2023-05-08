use frame_support::pallet_prelude::*;
use sp_application_crypto::{ecdsa, ed25519, sr25519, KeyTypeId, RuntimePublic};
use sp_runtime::Percent;
use sp_std::prelude::*;

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
}

impl AttesterInfo {
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
//
// impl<Account, Balance> AttestersReadApi<Account, Balance>
//     for AttestersReadApiEmptyMock<Account, Balance>
// {
//     fn previous_committee() -> Vec<Account> {
//         vec![]
//     }
//
//     fn current_committee() -> Vec<Account> {
//         vec![]
//     }
//
//     fn active_set() -> Vec<Account> {
//         vec![]
//     }
//
//     fn honest_active_set() -> Vec<Account> {
//         vec![]
//     }
//
//     fn read_nominations(for_attester: &Account) -> Vec<(Account, Balance)> {
//         vec![]
//     }
// }

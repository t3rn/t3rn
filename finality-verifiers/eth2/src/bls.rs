use crate::{types, Config, Error};
use frame_support::ensure;
use milagro_bls::AggregatePublicKey;
use types::{BLSPubkey, BLSSignature, Bytes};

pub fn fast_aggregate_verify<T: Config>(
    bls_ssz_pubkeys: Vec<BLSPubkey>,
    message: Bytes,
    signature: BLSSignature,
) -> Result<bool, Error<T>> {
    match milagro_bls::AggregateSignature::from_bytes(&signature[..]) {
        Ok(sig) => {
            let mut public_keys_g1 = vec![];
            for bls_ssz_pubkey in &bls_ssz_pubkeys {
                public_keys_g1.push(
                    milagro_bls::PublicKey::from_bytes(&bls_ssz_pubkey[..])
                        .map_err(|_| Error::<T>::InvalidBLSPublicKeyUsedForVerification)?,
                );
            }
            let agg_pub_key = AggregatePublicKey::into_aggregate(&public_keys_g1)
                .map_err(|_| Error::<T>::InvalidBLSPublicKeyUsedForVerification)?;
            ensure!(
                sig.fast_aggregate_verify_pre_aggregated(&message[..], &agg_pub_key),
                Error::<T>::InvalidBLSSignature
            );
            Ok(true)
        },
        Err(_) => Err(Error::<T>::InvalidBLSSignature),
    }
}

#[cfg(feature = "testing")]
#[cfg(test)]
pub mod mlc_bls_test {
    use super::*;
    use crate::mock::{run_test, TestRuntime};
    use codec::Encode;
    use frame_support::assert_ok;
    use hex_literal::hex;

    use sp_std::iter::FromIterator;

    #[test]
    pub fn mlc_bls_fast_aggregate_verify_minimal() {
        run_test(|| {
            assert_ok!(crate::bls::fast_aggregate_verify::<TestRuntime>(
                vec![
                    BLSPubkey::from_iter(hex!("a73eb991aa22cdb794da6fcde55a427f0a4df5a4a70de23a988b5e5fc8c4d844f66d990273267a54dd21579b7ba6a086").encode()),
                    BLSPubkey::from_iter(hex!("b29043a7273d0a2dbc2b747dcf6a5eccbd7ccb44b2d72e985537b117929bc3fd3a99001481327788ad040b4077c47c0d").encode()),
                    BLSPubkey::from_iter(hex!("b928f3beb93519eecf0145da903b40a4c97dca00b21f12ac0df3be9116ef2ef27b2ae6bcd4c5bc2d54ef5a70627efcb7").encode()),
                    BLSPubkey::from_iter(hex!("9446407bcd8e5efe9f2ac0efbfa9e07d136e68b03c5ebc5bde43db3b94773de8605c30419eb2596513707e4e7448bb50").encode()),
                ],
                hex!("69241e7146cdcc5a5ddc9a60bab8f378c0271e548065a38bcc60624e1dbed97f").into(),
                hex!("b204e9656cbeb79a9a8e397920fd8e60c5f5d9443f58d42186f773c6ade2bd263e2fe6dbdc47f148f871ed9a00b8ac8b17a40d65c8d02120c00dca77495888366b4ccc10f1c6daa02db6a7516555ca0665bca92a647b5f3a514fa083fdc53b6e"),
		    ));
        });
    }
}

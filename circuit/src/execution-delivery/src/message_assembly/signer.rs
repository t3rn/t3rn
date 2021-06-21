#![cfg_attr(not(feature = "std"), no_std)]

pub mod app {
    use sp_application_crypto::{app_crypto, sr25519};
    pub const CIRCUIT_CRYPTO_ID: sp_application_crypto::KeyTypeId =
        sp_application_crypto::KeyTypeId(*b"circ");
    app_crypto!(sr25519, CIRCUIT_CRYPTO_ID);
}
/// Generates the extrinsic's call field for a given module and call passed as &str
/// # Arguments
///
/// * 'node_metadata' - This crate's parsed node metadata as field of the API.
/// * 'module' - Module name as &str for which the call is composed.
/// * 'call' - Call name as &str
/// * 'args' - Optional sequence of arguments of the call. They are not checked against the metadata.
/// As of now the user needs to check himself that the correct arguments are supplied.
#[macro_export]
macro_rules! compose_call {
($node_metadata: expr, $module: expr, $call_name: expr $(, $args: expr) *) => {
        {
            let (module_index, call_index) = $node_metadata.lookup_module_and_call_indices($module, $call_name).unwrap();

            ([module_index as u8, call_index as u8] $(, ($args)) *)
        }
    };
}

/// Generates an Unchecked extrinsic for a given call
/// # Arguments
///
/// * 'signer' - AccountKey that is used to sign the extrinsic.
/// * 'call' - call as returned by the compose_call! macro or via substrate's call enums.
/// * 'nonce' - signer's account nonce: u32
/// * 'era' - Era for extrinsic to be valid
/// * 'genesis_hash' - sp-runtime::Hash256/[u8; 32].
/// * 'runtime_spec_version' - RuntimeVersion.spec_version/u32
#[macro_export]
macro_rules! compose_extrinsic_offline {
    ($signer: expr,
    $call: expr,
    $nonce: expr,
    $era: expr,
    $genesis_hash: expr,
    $genesis_or_current_hash: expr,
    $runtime_spec_version: expr,
    $transaction_version: expr) => {{
        //     use t3rn_primitives::*;
        //
        //     let extra = GenericExtra::new($era, $nonce);
        //
        //     let raw_payload = SignedPayload::from_raw(
        //         $call.clone(),
        //         extra.clone(),
        //         (
        //             $runtime_spec_version,
        //             $transaction_version,
        //             $genesis_hash,
        //             $genesis_or_current_hash,
        //             (),
        //             (),
        //             (),
        //         ),
        //     );
        //
        //     let signature = raw_payload.using_encoded(|payload| $signer.sign(payload));
        //
        //     let mut arr = Default::default();
        //     arr.clone_from_slice($signer.public().as_ref());
        //
        //     UncheckedExtrinsicV4::new_signed(
        //         $call,
        //         GenericAddress::from(AccountId::from(arr)),
        //         signature.into(),
        //         extra,
        //     )
        vec![]
    }};
}

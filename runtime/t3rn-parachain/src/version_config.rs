


#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    // https://docs.rs/sp-version/latest/sp_version/struct.RuntimeVersion.html
    spec_name: create_runtime_str!("t0rn"),
    impl_name: create_runtime_str!("Circuit Collator"),
    authoring_version: 2,
    spec_version: 3,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 2,
    // https://github.com/paritytech/cumulus/issues/998
    // https://github.com/paritytech/substrate/pull/9732
    // https://github.com/paritytech/substrate/pull/10073
    state_version: 1, // 0 = old, 1 = new; see above 4 details
};

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}



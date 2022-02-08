//! t3rn service. Specialized wrapper over substrate service.

pub mod chain_spec;

mod rpc;

#[cfg(feature = "with-standalone-runtime")]
pub mod standalone;

#[cfg(feature = "with-parachain-runtime")]
pub mod parachain;

pub trait IdentifyVariant {
    fn is_standalone(&self) -> bool;
    fn is_parachain(&self) -> bool;
}

impl IdentifyVariant for Box<dyn sc_service::ChainSpec> {
    fn is_standalone(&self) -> bool {
        self.id().starts_with("standalone")
    }

    fn is_parachain(&self) -> bool {
        self.id().starts_with("parachain")
    }
}

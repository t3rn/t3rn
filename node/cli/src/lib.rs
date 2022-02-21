#[cfg(feature = "with-parachain-runtime")]
pub mod parachain;

#[cfg(feature = "with-standalone-runtime")]
pub mod standalone;

pub mod command;

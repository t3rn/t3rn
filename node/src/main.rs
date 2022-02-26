//! t3rn Parachain Node CLI
#![warn(missing_docs)]

#[cfg(feature = "with-standalone-runtime")]
use circuit_cli::command::standalone::run;

#[cfg(feature = "with-parachain-runtime")]
use circuit_cli::command::parachain::run;

fn main() -> sc_cli::Result<()> {
    run()
}

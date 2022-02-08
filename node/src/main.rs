//! t3rn Parachain Node CLI

#![warn(missing_docs)]

fn main() -> sc_cli::Result<()> {
    circuit_cli::command::run()
}

//! t3rn Parachain Node CLI

#![warn(missing_docs)]

#[macro_use]

mod cli;
mod command;

fn main() -> sc_cli::Result<()> {
    command::run()
}

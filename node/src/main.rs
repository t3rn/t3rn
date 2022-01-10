//! t3rn Parachain Node CLI

#![warn(missing_docs)]

#[macro_use]
use circuit_service;
mod cli;
mod command;

fn main() -> sc_cli::Result<()> {
    command::run()
}

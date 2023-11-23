//! Substrate Node Template CLI library.
use t0rn_parachain_collator::command;

fn main() -> sc_cli::Result<()> {
    command::run()
}

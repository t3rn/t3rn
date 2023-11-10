//! Substrate Node Template CLI library.
use t3rn_parachain_collator::command;

fn main() -> sc_cli::Result<()> {
    command::run()
}

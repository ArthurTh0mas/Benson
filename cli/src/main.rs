//! Benson Box cli entrypoint
use benson_cli::command;
use sc_cli;

fn main() -> sc_cli::Result<()> {
	command::run()
}

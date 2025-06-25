use clap::Parser;

use impl_cli_main::impl_cli_main;

impl_cli_main!();

fn run(args: &CliArgs) -> Result<(), RunError> {
    match args {
        CliArgs {} => Ok(()),
    }
}

#[derive(Parser)]
struct CliArgs {}

#[derive(Debug, thiserror::Error)]
enum RunError {}

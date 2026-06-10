////////////////////////////////////////////////////////////////////////////////////////////////////

use anyhow::Result as anyResult;
use clap::{CommandFactory, Parser};
use clap_complete::{Shell as ClapShell, generate};
use std::io::{self};
use std::process;

////////////////////////////////////////////////////////////////////////////////////////////////////

use lib::cli;
use lib::core;

////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() -> anyResult<()> {
    let cli = cli::Cli::parse();

    match cli.command {
        None => {
            core::run(cli)?;
        }
        Some(cli::Commands::Identity) => {
            println!("{}", lib::IDENTITY);
        }
        Some(cli::Commands::Completion { shell }) => {
            let shell = match shell {
                cli::Shell::Bash => ClapShell::Bash,
                cli::Shell::Zsh => ClapShell::Zsh,
                cli::Shell::Fish => ClapShell::Fish,
                cli::Shell::Powershell => ClapShell::PowerShell,
            };
            let mut cmd = cli::Cli::command();
            let bin_name = env!("CARGO_PKG_NAME");
            generate(shell, &mut cmd, bin_name, &mut io::stdout());
            process::exit(0);
        }
    }
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

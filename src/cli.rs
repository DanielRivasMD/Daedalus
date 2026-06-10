////////////////////////////////////////////////////////////////////////////////////////////////////

use clap::{Parser, Subcommand, ValueEnum};

////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::HELP;
use crate::core;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    before_help = concat!(env!("CARGO_PKG_AUTHORS"), "\n", env!("CARGO_PKG_NAME"), " v", env!("CARGO_PKG_VERSION")),
    long_about = HELP,
)]
pub struct Cli {
    /// Subcommand (optional)
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Items to be forged (optional input directory)
    #[arg(long)]
    pub r#in: Option<String>,

    /// Path to forge
    #[arg(long)]
    pub out: Option<String>,

    /// Forge components (one or more)
    #[arg(long, num_args = 1.., value_name = "FILE")]
    pub files: Option<Vec<String>>,

    /// Replacement in form old=new, space-separated.
    /// Append :line for whole line replacement, :token for token replacement (default)
    #[arg(long, value_name = "OLD=NEW[:mode]")]
    pub replace: Vec<core::Replacement>,

    /// Enable verbose diagnostics
    #[arg(short = 'v', long)]
    pub verbose: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Subcommand)]
pub enum Commands {
    /// Print identity
    Identity,
    /// Generate shell completions
    Completion {
        /// Shell for which to generate completions
        #[arg(value_enum)]
        shell: Shell,
    },
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, ValueEnum)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    Powershell,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////////

use clap::Parser;

////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::HELP;
use crate::custom::Replacement;

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
    pub replace: Vec<Replacement>,

    /// Enable verbose diagnostics
    #[arg(short = 'v', long)]
    pub verbose: bool,

    /// Show identity / myth
    #[arg(long)]
    pub identity: bool,

    /// Generate shell completions (bash, zsh, fish, powershell)
    #[arg(long, value_name = "SHELL")]
    pub completion: Option<String>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

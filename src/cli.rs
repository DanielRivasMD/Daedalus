////////////////////////////////////////////////////////////////////////////////////////////////////

use clap::{Parser, Subcommand, ValueEnum};

////////////////////////////////////////////////////////////////////////////////////////////////////

const HELP: &str = r"Command line file forger

Examples:
  dd --files header.html footer.html --out output/page.html
  dd --files config.tmpl --out config.json --replace VERSION=1.0.0 API_URL=https://api.example.com
  dd --in templates/ --files base.tmpl nav.tmpl --out build/index.html --replace {{YEAR}}=2026:token {{AUTHOR}}=Daniel:token
  dd --files script.js --out dist/script.min.js --replace console.log=:line
  dd --files README.md --out README.md --replace v0.0.0=v1.2.3";

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Parser)]
#[command(
    name = env!("CARGO_BIN_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    before_help = concat!(env!("CARGO_PKG_AUTHORS"), "\n", env!("CARGO_PKG_NAME"), " v", env!("CARGO_PKG_VERSION")),
    long_about = HELP,
)]
pub struct Cli {
    /// Subcommand (optional)
    #[command(subcommand)]
    pub command: Option<Command>,

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
    pub replace: Vec<issac::Replacement>,

    /// Enable verbose diagnostics
    #[arg(short = 'v', long)]
    pub verbose: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Subcommand)]
pub enum Command {
    /// Print identity
    #[command(hide = true)]
    #[command(aliases = &["id"])]
    Identity,

    /// Generate shell completions
    #[command(hide = true)]
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

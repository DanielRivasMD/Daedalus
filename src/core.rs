////////////////////////////////////////////////////////////////////////////////////////////////////

use anyhow::Result as anyResult;
use std::process;

////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::cli;
use crate::util;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct Replacement {
    pub old: String,
    pub new: String,
    pub mode: String, // "token" or "line"
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl std::str::FromStr for Replacement {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (old, rest) = s.split_once('=').ok_or("invalid replace pair")?;
        let (mode, new_val) = if let Some(stripped) = rest.strip_suffix(":line") {
            ("line", stripped)
        } else if let Some(stripped) = rest.strip_suffix(":token") {
            ("token", stripped)
        } else {
            ("token", rest)
        };

        Ok(Replacement {
            old: old.to_string(),
            new: new_val.to_string(),
            mode: mode.to_string(),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn run(cli: cli::Cli) -> anyResult<()> {
    // Forging mode – validate required flags
    let out = cli.out.unwrap_or_else(|| {
        eprintln!("error: the following required arguments were not provided:");
        eprintln!("  --out <OUT>");
        eprintln!();
        eprintln!(
            "Usage: {} --out <OUT> --files <FILE>...",
            env!("CARGO_BIN_NAME")
        );
        eprintln!();
        eprintln!("For more information, try '--help'.");
        process::exit(2);
    });

    let mut files = cli.files.unwrap_or_else(|| {
        eprintln!("error: the following required arguments were not provided:");
        eprintln!("  --files <FILE>...");
        eprintln!();
        eprintln!(
            "Usage: {} --out <OUT> --files <FILE>...",
            env!("CARGO_BIN_NAME")
        );
        eprintln!();
        eprintln!("For more information, try '--help'.");
        process::exit(2);
    });

    // Normalize paths
    let (in_path, out_dir, out_file) = util::normalize_paths(cli.r#in.as_deref(), &mut files, &out);

    // Run the main forging operation
    if let Err(err) = util::cat_files(
        in_path,
        out_dir,
        &out_file,
        &files,
        &cli.replace,
        cli.verbose,
    ) {
        eprintln!("error: {:#}", err);
        process::exit(2);
    }
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

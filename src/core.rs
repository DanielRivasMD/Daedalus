////////////////////////////////////////////////////////////////////////////////////////////////////

use anyhow::Result as anyResult;
use std::process;

////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::cli;
use crate::util;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn run(cli: cli::Cli) -> anyResult<()> {
    // Forging mode - validate required flags
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

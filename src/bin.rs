////////////////////////////////////////////////////////////////////////////////////////////////////

use std::io::{self};
use std::process;

use clap::{CommandFactory, Parser};
use clap_complete::{Shell, generate};

////////////////////////////////////////////////////////////////////////////////////////////////////

use lib::cli::*;
use lib::util::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {
    let cli = Cli::parse();

    // Handle --identity
    if cli.identity {
        println!("{}", lib::IDENTITY);
        process::exit(0);
    }

    // Handle --completion
    if let Some(shell) = cli.completion {
        let shell = match shell.to_lowercase().as_str() {
            "bash" => Shell::Bash,
            "zsh" => Shell::Zsh,
            "fish" => Shell::Fish,
            "powershell" => Shell::PowerShell,
            other => {
                eprintln!("unsupported shell: {}", other);
                process::exit(2);
            }
        };
        let mut cmd = Cli::command();
        let bin_name = "mbombo";
        generate(shell, &mut cmd, bin_name, &mut io::stdout());
        process::exit(0);
    }

    let out = cli.out.unwrap_or_else(|| {
        eprintln!("error: the following required arguments were not provided:");
        eprintln!("  --out <OUT>");
        eprintln!();
        eprintln!("Usage: mbombo --out <OUT> --files <FILE>...");
        eprintln!();
        eprintln!("For more information, try '--help'.");
        process::exit(2);
    });

    let mut files = cli.files.unwrap_or_else(|| {
        eprintln!("error: the following required arguments were not provided:");
        eprintln!("  --files <FILE>...");
        eprintln!();
        eprintln!("Usage: mbombo --out <OUT> --files <FILE>...");
        eprintln!();
        eprintln!("For more information, try '--help'.");
        process::exit(2);
    });

    // Normalize paths
    let (in_path, out_dir, out_file) = normalize_paths(cli.r#in.as_deref(), &mut files, &out);

    // Run the main forging operation
    if let Err(err) = cat_files(
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
}

////////////////////////////////////////////////////////////////////////////////////////////////////

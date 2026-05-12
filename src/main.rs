////////////////////////////////////////////////////////////////////////////////////////////////////

use std::fs;
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process;

use anyhow::{Context, Result};
use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};
use regex::Regex;

////////////////////////////////////////////////////////////////////////////////////////////////////

const IDENTITY: &str = r#"
Mbombo, also called Bumba, is the creator god in the religion and mythology of the Kuba people of Central Africa in the area that is now known as Democratic Republic of the Congo

In the Mbombo creation myth, Mbombo was a giant in form and white in color. The myth describes the creation of the universe from nothing

Role: Mbombo is considered a creator god in Bushongo mythology
Creation story: According to legend, in the beginning, there was only darkness and water, and Mbombo was the only being. He was a giant, pale god who eventually felt pain in his stomach and vomited up the sun, moon, stars, and then the Earth itself, including animals and people
Symbolism: His creation of the world through vomiting is unique and symbolic, often interpreted as a metaphor for creative force through suffering or sacrifice
Assistants: After creation, Mbombo delegated tasks to his sons and some of the first humans and animals to help finish shaping the world
"#;

const HELP: &str = r"Command line file forger

Examples:
  mbombo --files header.html footer.html --out output/page.html
  mbombo --files config.tmpl --out config.json --replace VERSION=1.0.0 API_URL=https://api.example.com
  mbombo --in templates/ --files base.tmpl nav.tmpl --out build/index.html --replace {{YEAR}}=2026:token {{AUTHOR}}=Daniel:token
  mbombo --files script.js --out dist/script.min.js --replace console.log=:line
  mbombo --files README.md --out README.md --replace v0.0.0=v1.2.3";

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
struct Cli {
    /// Items to be forged (optional input directory)
    #[arg(long)]
    r#in: Option<String>,

    /// Path to forge
    #[arg(long)]
    out: Option<String>,

    /// Forge components (one or more)
    #[arg(long, num_args = 1.., value_name = "FILE")]
    files: Option<Vec<String>>,

    /// Replacement in form old=new, space-separated.
    /// Append :line for whole line replacement, :token for token replacement (default)
    #[arg(long, value_name = "OLD=NEW[:mode]")]
    replace: Vec<Replacement>,

    /// Enable verbose diagnostics
    #[arg(short = 'v', long)]
    verbose: bool,

    /// Show identity / myth
    #[arg(long)]
    identity: bool,

    /// Generate shell completions (bash, zsh, fish, powershell)
    #[arg(long, value_name = "SHELL")]
    completion: Option<String>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
struct Replacement {
    old: String,
    new: String,
    mode: String, // "token" or "line"
}

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

fn apply_replacements(content: &str, replacements: &[Replacement]) -> String {
    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

    for rep in replacements {
        let re = if rep.mode == "line" {
            let pattern = format!(r"\b{}\b", regex::escape(&rep.old));
            Regex::new(&pattern).ok()
        } else {
            None
        };

        for line in &mut lines {
            match rep.mode.as_str() {
                "line" => {
                    if let Some(ref regex) = re {
                        if regex.is_match(line) {
                            *line = rep.new.clone();
                        }
                    }
                }
                _ => {
                    // token mode (default)
                    *line = line.replace(&rep.old, &rep.new);
                }
            }
        }
    }

    lines.join("\n")
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Normalize the flags:
/// - If only one input file and it contains a path separator, extract its directory
///   as `in_path` and leave the file name as the sole input file
/// - Split the output path into `out_dir` and `out_file`
fn normalize_paths(
    in_flag: Option<&str>,
    files: &mut [String],
    out_flag: &str,
) -> (Option<PathBuf>, PathBuf, String) {
    // Determine in_path from --in or from first file
    let mut in_path = in_flag.map(PathBuf::from);
    if files.len() == 1 {
        let full = Path::new(&files[0]);
        if full.parent().is_some_and(|p| p != Path::new(""))
            && files[0].contains(std::path::MAIN_SEPARATOR)
        {
            in_path = full.parent().map(|p| p.to_path_buf());
            // replace the file with its base name
            if let Some(fname) = full.file_name().and_then(|s| s.to_str()) {
                files[0] = fname.to_string();
            }
        }
    }

    // Determine out_dir and out_file
    let out = Path::new(out_flag);
    let (out_dir, out_file) = if out.parent().is_some_and(|p| p != Path::new(""))
        && out_flag.contains(std::path::MAIN_SEPARATOR)
    {
        (
            out.parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| PathBuf::from(".")),
            out.file_name()
                .and_then(|s| s.to_str())
                .expect("invalid output file name")
                .to_string(),
        )
    } else {
        (PathBuf::from("."), out_flag.to_string())
    };

    (in_path, out_dir, out_file)
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn cat_files(
    in_path: Option<PathBuf>,
    out_dir: PathBuf,
    out_file: &str,
    files: &[String],
    replacements: &[Replacement],
    verbose: bool,
) -> Result<()> {
    let base_dir = in_path.unwrap_or_else(|| PathBuf::from("."));
    let out_full = out_dir.join(out_file);

    // Overwrite conflict detection
    let (overwrite_single, source_files) = if files.len() == 1 && files[0] == out_file {
        // single file overwrite: copy to temp
        let tmp_file = format!("{}.tmp", out_file);
        let tmp_full = out_dir.join(&tmp_file);

        if verbose {
            eprintln!(
                "verbose: copying {} to temp {}",
                out_full.display(),
                tmp_full.display()
            );
        }
        fs::copy(&out_full, &tmp_full).with_context(|| {
            format!(
                "failed to copy {} to {}",
                out_full.display(),
                tmp_full.display()
            )
        })?;
        let src_paths = vec![tmp_full.clone()];
        (true, (src_paths, tmp_full))
    } else if files.len() > 1 && files.contains(&out_file.to_string()) {
        anyhow::bail!(
            "cannot overwrite output when multiple input files are used (outpath: {}, files: {:?})",
            out_full.display(),
            files
        );
    } else {
        // normal: join each file with in_path
        let src_paths: Vec<PathBuf> = files.iter().map(|f| base_dir.join(f)).collect();
        (false, (src_paths, PathBuf::new())) // second element not used
    };

    let (source_paths, tmp_cleanup) = (source_files.0, source_files.1);

    // Remove existing output file if present
    if out_full.exists() {
        if verbose {
            eprintln!("verbose: removing existing out file {}", out_full.display());
        }
        fs::remove_file(&out_full).with_context(|| {
            format!("failed to remove existing out file {}", out_full.display())
        })?;
    }

    // Open output file for writing
    let fwrite = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&out_full)
        .with_context(|| format!("failed to open out file {}", out_full.display()))?;
    let mut writer = BufWriter::new(fwrite);

    // Concatenate and apply replacements
    for src_path in &source_paths {
        if verbose {
            eprintln!("verbose: reading source {}", src_path.display());
        }
        let raw = fs::read_to_string(src_path)
            .with_context(|| format!("failed to read source file {}", src_path.display()))?;

        let content = apply_replacements(&raw, replacements);
        let trimmed = content.trim_end_matches('\n');
        writeln!(writer, "{}", trimmed)
            .with_context(|| format!("failed to write to out file {}", out_full.display()))?;
    }

    writer
        .flush()
        .with_context(|| format!("failed to flush writer for {}", out_full.display()))?;

    // Clean up temp file if overwrite single
    if overwrite_single && !tmp_cleanup.as_os_str().is_empty() {
        if verbose {
            eprintln!("verbose: removing temp file {}", tmp_cleanup.display());
        }
        let _ = fs::remove_file(tmp_cleanup);
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {
    let cli = Cli::parse();

    // Handle --identity
    if cli.identity {
        println!("{}", IDENTITY);
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

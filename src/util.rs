////////////////////////////////////////////////////////////////////////////////////////////////////

use anyhow::{Context, Result};
use std::fs;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Normalize the flags:
/// - If only one input file and it contains a path separator, extract its directory
///   as `in_path` and leave the file name as the sole input file
/// - Split the output path into `out_dir` and `out_file`
pub fn normalize_paths(
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

pub fn cat_files(
    in_path: Option<PathBuf>,
    out_dir: PathBuf,
    out_file: &str,
    files: &[String],
    replacements: &[issac::Replacement],
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

        let content = issac::apply_replacements(&raw, replacements);
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

use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use clap::{IntoApp, Parser};
use fforget::{fcached, fforget};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Paths to the files to attempt to remove from the page cache.
    files: Vec<PathBuf>,

    /// Skip the final check on whether files still reside in memory.
    ///
    /// This flag can be used to fforget on a best-effort basis, without inducing a non-zero exit
    /// status (and an accompanying error message on stderr) in case of failures.
    #[clap(short, long)]
    no_check: bool,

    /// Only check whether the files at the provided paths appear to be in the page cache, without
    /// hinting Linux to remove them if they do.
    #[clap(short, long)]
    check_only: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    if cli.check_only && cli.no_check {
        Cli::into_app()
            .error(
                clap::ErrorKind::ArgumentConflict,
                "Flags '--check-only' and '--no-check' cannot coexist",
            )
            .exit();
    }

    if !cli.check_only {
        cli.files.iter().try_for_each(|path| {
            fforget(path).with_context(|| format!("failed to fforget file '{}'", path.display()))
        })?;
    }

    if !cli.no_check {
        let mut still_cached = Vec::with_capacity(cli.files.len());
        for path in cli.files.iter() {
            let cached = fcached(path).with_context(|| {
                format!(
                    "failed to check whether file '{}' still resides in memory",
                    path.display()
                )
            })?;
            if cached {
                still_cached.push(path);
            }
        }
        if !still_cached.is_empty() {
            bail!(
                "the following files still appear to reside in memory (possibly partially): {:#?}",
                still_cached,
            );
        }
    }

    Ok(())
}

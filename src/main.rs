use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use fforget::{fcached, fforget};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    files: Vec<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    for path in cli.files.iter() {
        fforget(path).with_context(|| format!("failed to fforget file '{}'", path.display()))?;
    }
    for path in cli.files.iter() {
        fcached(path).with_context(|| {
            format!(
                "file '{}' still resides in page cache (possibly partially)",
                path.display()
            )
        })?;
    }
    Ok(())
}

/*
 * Test failure:
 *
 *     $ ./target/release/fforget-bin /proc/cmdline
 *     Error: file '/proc/cmdline' still resides in page cache (possibly partially)
 *
 *     Caused by:
 *         0: mmap(2) failed
 *         1: Invalid argument (os error 22)
 *
 *  and
 *
 *     $ ./target/release/fforget-bin /proc/slabinfo
 *     Error: failed to fforget file '/proc/slabinfo'
 *
 *     Caused by:
 *         0: Failed to open(2) file "/proc/slabinfo"
 *         1: Permission denied (os error 13)
 */
